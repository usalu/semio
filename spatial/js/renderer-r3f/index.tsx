// #region 🧲Header
/** @emoji 🎬 `@spatial/js-renderer-r3f` — R3F `InteractionDisplay`, ground picking, interaction adapter, `InteractionCanvas`, and snapshot hooks. See `spatial/assets/interactions/box.interaction.json`. */
// #endregion 🧲Header

// #region 📥Imports
import { Line, OrbitControls, Text } from "@react-three/drei";
import { Canvas, type ThreeEvent } from "@react-three/fiber";
import { useCallback, useEffect, useMemo, useRef, useState, useSyncExternalStore, type KeyboardEvent, type ReactNode } from "react";
import { MOUSE } from "three";
import * as THREE from "three";

THREE.Object3D.DEFAULT_UP.set(0, 0, 1);

import {
	abortActiveInteractionSession,
	applyTopologyDiff,
	boxTopologyDiff,
	buildBoxInteractionSpec,
	cellRef,
	createInteractionRuntime,
	DerivedViewService,
	evaluateAnchorPosition,
	topologyCellAabb,
	DocumentHistory,
	arcEndOnCircle,
	arcSamplePoints,
	edgeSamplePoints,
	isInteractionSessionActive,
	parseTopologyGraphJson,
	listKeyedInteractionTransitions,
	meshFaceTopologyDiff,
	resolveSpatialInteractionKey,
	TopologyGraph,
	type InteractionEvent,
	type InteractionKeybindRow,
	type InteractionRuntime,
	type InteractionRuntimeOptions,
	type InteractionSnapshot,
	type AnchorRecord,
	type CellComplexRecord,
	type CellRecord,
	type ClusterRecord,
	type InteractionSpec,
	type DisplayItem,
	type DisplayModel,
	type EdgeRecord,
	type FaceRecord,
	type KernelAdapter,
	type ModelDocument,
	type PartView,
	type SelectionTarget,
	type ShellRecord,
	type MeshPreview,
	type SpatialInteraction,
	type TopologyEntityKind,
	type TopologyGraphJson,
	type Vec3,
	type VertexRecord,
	type WireRecord,
} from "@spatial/js-core";
// #endregion 📥Imports

// #region 🪩ArchivedFootprints
/** @emoji 📦 Footprint of a finished axis-aligned box for persistent REPL overlays. */
export interface ArchivedBoxLayout {
	readonly cornerA: Vec3;
	readonly cornerB: Vec3;
	readonly height: number;
}

function isVec3Record(v: unknown): v is Vec3 {
	return Array.isArray(v) && v.length === 3 && v.every((x) => typeof x === "number");
}

/** @emoji 📦 Reads `origin`/`corner`/`height` from post-commit interaction context when present. */
export function tryArchivedBoxFromContext(ctx: Record<string, unknown>): ArchivedBoxLayout | null {
	const o = ctx.origin;
	const c = ctx.corner;
	const h = ctx.height;
	if (!isVec3Record(o) || !isVec3Record(c)) return null;
	const hz = typeof h === "number" && Number.isFinite(h) && h > 0 ? h : null;
	if (hz === null) return null;
	return { cornerA: o, cornerB: c, height: hz };
}

function mergeDisplayWithArchivedBoxes(base: DisplayModel, archived: readonly ArchivedBoxLayout[]): DisplayModel {
	if (archived.length === 0) return base;
	const extra: DisplayItem[] = archived.map((b, i) => ({
		kind: "box-preview",
		id: `archived-box-${i}`,
		role: "archived",
		params: { cornerA: b.cornerA, cornerB: b.cornerB, height: b.height },
	}));
	return { ...base, items: [...extra, ...base.items] };
}

function archivedBoxesFromHistory(history: DocumentHistory): readonly ArchivedBoxLayout[] {
	return history
		.entries()
		.map((mod) => (mod.result.archiveContext ? tryArchivedBoxFromContext(mod.result.archiveContext) : null))
		.filter((box): box is ArchivedBoxLayout => box !== null);
}

function replBaseDisplayForHistory(snapshot: InteractionSnapshot): DisplayModel {
	if (snapshot.state !== "committed") return snapshot.display;
	return { ...snapshot.display, items: snapshot.display.items.filter((item) => item.role !== "preview") };
}
// #endregion 🪩ArchivedFootprints

// #region 📐Layout
/** @emoji 📐 Center and axis-aligned scale for a unit `BoxGeometry` from two XY footprint corners and height. */
export function computeBoxPreviewLayout(
	cornerA: Vec3,
	cornerB: Vec3,
	height: number,
): { readonly position: Vec3; readonly scale: Vec3 } {
	const ax = Math.min(cornerA[0], cornerB[0]);
	const ay = Math.min(cornerA[1], cornerB[1]);
	const bx = Math.max(cornerA[0], cornerB[0]);
	const by = Math.max(cornerA[1], cornerB[1]);
	const w = bx - ax;
	const d = by - ay;
	const h = height;
	const minZ = Math.min(cornerA[2], cornerB[2]);
	const cx = (ax + bx) / 2;
	const cy = (ay + by) / 2;
	const ez = 1e-9;
	return {
		position: [cx, cy, minZ + h / 2],
		scale: [Math.max(w, ez), Math.max(d, ez), Math.max(h, ez)],
	};
}

function readVec3(v: unknown): Vec3 | null {
	if (Array.isArray(v) && v.length === 3 && v.every((x) => typeof x === "number")) return v as unknown as Vec3;
	return null;
}

function readNumber(v: unknown): number | null {
	return typeof v === "number" && Number.isFinite(v) ? v : null;
}

function readVec3Array(v: unknown): readonly Vec3[] {
	if (!Array.isArray(v)) return [];
	return v.filter(isVec3Record) as readonly Vec3[];
}

function vec3Sub(a: Vec3, b: Vec3): Vec3 {
	return [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
}

function vec3Add(a: Vec3, b: Vec3): Vec3 {
	return [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
}

function translateVec3(p: Vec3, delta: Vec3): Vec3 {
	return vec3Add(p, delta);
}

/** @emoji 📦 Axis-aligned bounds for topology highlight wireframes. */
export function bboxFromPoints(points: readonly Vec3[]): { readonly min: Vec3; readonly max: Vec3 } | null {
	if (!points.length) return null;
	let minX = points[0]![0];
	let minY = points[0]![1];
	let minZ = points[0]![2];
	let maxX = minX;
	let maxY = minY;
	let maxZ = minZ;
	for (const p of points) {
		minX = Math.min(minX, p[0]);
		minY = Math.min(minY, p[1]);
		minZ = Math.min(minZ, p[2]);
		maxX = Math.max(maxX, p[0]);
		maxY = Math.max(maxY, p[1]);
		maxZ = Math.max(maxZ, p[2]);
	}
	const pad = 0.04;
	return {
		min: [minX - pad, minY - pad, minZ - pad],
		max: [maxX + pad, maxY + pad, maxZ + pad],
	};
}

/** @emoji 📦 Twelve edges of an axis-aligned box for preview line rendering. */
export function bboxWireSegments(min: Vec3, max: Vec3): readonly (readonly [Vec3, Vec3])[] {
	const [x0, y0, z0] = min;
	const [x1, y1, z1] = max;
	const c: readonly Vec3[] = [
		[x0, y0, z0],
		[x1, y0, z0],
		[x1, y1, z0],
		[x0, y1, z0],
		[x0, y0, z1],
		[x1, y0, z1],
		[x1, y1, z1],
		[x0, y1, z1],
	];
	const idx: readonly (readonly [number, number])[] = [
		[0, 1],
		[1, 2],
		[2, 3],
		[3, 0],
		[4, 5],
		[5, 6],
		[6, 7],
		[7, 4],
		[0, 4],
		[1, 5],
		[2, 6],
		[3, 7],
	];
	return idx.map(([a, b]) => [c[a]!, c[b]!] as const);
}

function parseDisplaySelectionTargets(v: unknown): readonly { readonly kind: TopologyEntityKind; readonly id: string }[] {
	if (!Array.isArray(v)) return [];
	const out: { kind: TopologyEntityKind; id: string }[] = [];
	for (const raw of v) {
		if (!raw || typeof raw !== "object") continue;
		const o = raw as Record<string, unknown>;
		const kind = o.kind;
		const id = o.id;
		if (typeof kind === "string" && typeof id === "string") out.push({ kind: kind as TopologyEntityKind, id });
	}
	return out;
}

/** @emoji 🖼️ Maps declarative `previewKind` + params to a point transform for topology wireframes. */
export function transformPointsForPreviewKind(
	previewKind: string,
	params: Record<string, unknown>,
): (point: Vec3) => Vec3 {
	const identity = (point: Vec3) => point;
	const cursor = readVec3(params.cursor);
	const prevPoint = readVec3(params.prevPoint);
	const from = readVec3(params.from) ?? prevPoint;
	const center = readVec3(params.center) ?? readVec3Array(params.points)[0] ?? null;
	if (previewKind === "move-preview" || previewKind === "copy-preview") {
		if (!from || !cursor) return identity;
		const delta = vec3Sub(cursor, from);
		return (point) => translateVec3(point, delta);
	}
	if (previewKind === "mirror-preview") {
		const planeStart = readVec3(params.mirrorStart) ?? from;
		if (!planeStart || !cursor) return identity;
		const dx = cursor[0] - planeStart[0];
		const dy = cursor[1] - planeStart[1];
		const len = Math.hypot(dx, dy);
		if (len < 1e-9) return identity;
		const nx = -dy / len;
		const ny = dx / len;
		return (point) => {
			const vx = point[0] - planeStart[0];
			const vy = point[1] - planeStart[1];
			const dot = vx * nx + vy * ny;
			return [point[0] - 2 * dot * nx, point[1] - 2 * dot * ny, point[2]];
		};
	}
	if (previewKind === "rotate-preview") {
		const pivot = center ?? from;
		const ref = prevPoint;
		if (!pivot || !ref || !cursor) return identity;
		const a0 = Math.atan2(ref[1] - pivot[1], ref[0] - pivot[0]);
		const a1 = Math.atan2(cursor[1] - pivot[1], cursor[0] - pivot[0]);
		const ang = a1 - a0;
		const c = Math.cos(ang);
		const s = Math.sin(ang);
		return (point) => {
			const x = point[0] - pivot[0];
			const y = point[1] - pivot[1];
			return [pivot[0] + x * c - y * s, pivot[1] + x * s + y * c, point[2]];
		};
	}
	if (previewKind === "scale-preview" || previewKind === "scale1d-preview") {
		const origin = center ?? from ?? readVec3Array(params.points)[0];
		const ref = prevPoint ?? from;
		if (!origin || !ref || !cursor) return identity;
		const d0 = Math.hypot(ref[0] - origin[0], ref[1] - origin[1]);
		const d1 = Math.hypot(cursor[0] - origin[0], cursor[1] - origin[1]);
		const scale = d0 > 1e-9 ? d1 / d0 : 1;
		return (point) => [
			origin[0] + (point[0] - origin[0]) * scale,
			origin[1] + (point[1] - origin[1]) * scale,
			origin[2] + (point[2] - origin[2]) * scale,
		];
	}
	return identity;
}

function previewKindUsesTopologyWireframe(previewKind: string): boolean {
	return (
		previewKind === "selected-objects" ||
		previewKind === "move-preview" ||
		previewKind === "copy-preview" ||
		previewKind === "mirror-preview" ||
		previewKind === "rotate-preview" ||
		previewKind === "scale-preview" ||
		previewKind === "scale1d-preview" ||
		previewKind.endsWith("-selection") ||
		previewKind.startsWith("boolean-") ||
		previewKind === "highlight-curves" ||
		previewKind === "cutters" ||
		previewKind === "split-objects" ||
		previewKind === "trim-preview" ||
		previewKind === "extrusion" ||
		previewKind === "network-curves"
	);
}

const raycastNone: THREE.Object3D["raycast"] = () => undefined;
// #endregion 📐Layout

// #region 🧲TopologyTargets
export type SpatialPickKind = "pointer.down" | "pointer.move";

export type SpatialPickTargetKind = Extract<
	TopologyEntityKind,
	"anchor" | "vertex" | "edge" | "wire" | "face" | "shell" | "cell" | "cellComplex" | "cluster" | "surface" | "part"
>;

export type SpatialPickViewKind = "raw" | "analytic";

export const SPATIAL_RAW_PICK_TARGET_KINDS: readonly SpatialPickTargetKind[] = [
	"anchor",
	"vertex",
	"edge",
	"wire",
	"face",
	"shell",
	"cell",
	"cellComplex",
	"cluster",
];

export const SPATIAL_ANALYTIC_PICK_TARGET_KINDS: readonly SpatialPickTargetKind[] = [
	"surface",
	"part",
];

export const SPATIAL_PICK_TARGET_KINDS: readonly SpatialPickTargetKind[] = [
	...SPATIAL_RAW_PICK_TARGET_KINDS,
	...SPATIAL_ANALYTIC_PICK_TARGET_KINDS,
];

export type SpatialPickKindToggles = Partial<Record<SpatialPickTargetKind, boolean>>;

export interface SpatialPickTarget {
	readonly kind: SpatialPickTargetKind;
	readonly id: string;
	readonly point: Vec3;
	readonly points?: readonly Vec3[];
	readonly derivedFrom?: readonly { readonly kind: "face" | "cell"; readonly id: string }[];
	readonly exposure?: "external" | "internal";
	readonly overlap?: "none" | "difference" | "intersection";
}

export interface SpatialSelectionRequest {
	readonly targets: readonly SpatialPickTarget[];
	readonly point: Vec3;
	readonly client: { readonly x: number; readonly y: number };
	readonly modifiers: InteractionEvent["modifiers"];
}

export type SpatialPickGeometry = TopologyGraph | TopologyGraphJson;

export function spatialPickTargetKey(target: SpatialPickTarget): string {
	return `${target.kind}:${target.id}`;
}

function spatialSelectionTargetKey(target: SelectionTarget): string {
	return `${target.kind}:${target.id}`;
}

function defaultSpatialPickKindToggles(): Record<SpatialPickTargetKind, boolean> {
	return Object.fromEntries(SPATIAL_PICK_TARGET_KINDS.map((kind) => [kind, true])) as Record<SpatialPickTargetKind, boolean>;
}

function spatialPickViewKinds(viewKind: SpatialPickViewKind): readonly SpatialPickTargetKind[] {
	return viewKind === "raw" ? SPATIAL_RAW_PICK_TARGET_KINDS : SPATIAL_ANALYTIC_PICK_TARGET_KINDS;
}

function spatialPickViewKindSet(viewKind: SpatialPickViewKind): ReadonlySet<SpatialPickTargetKind> {
	return new Set(spatialPickViewKinds(viewKind));
}

export function filterSpatialPickTargetsByView(
	targets: readonly SpatialPickTarget[],
	viewKind: SpatialPickViewKind,
): SpatialPickTarget[] {
	const viewKinds = spatialPickViewKindSet(viewKind);
	return targets.filter((target) => viewKinds.has(target.kind));
}

function recordsById<T extends { id: string }>(xs: readonly T[]): Record<string, T> {
	const o: Record<string, T> = {};
	for (const x of xs) o[x.id] = x;
	return o;
}

function asRecordBucket<T extends { id: string }>(x: readonly T[] | Record<string, T> | undefined): Record<string, T> {
	if (!x) return {};
	return Array.isArray(x) ? recordsById(x) : x;
}

/** @emoji 🧲 Normalizes `TopologyGraphJson` array buckets to the record shape used by interaction math. */
function topologyGeometryBuckets(g: SpatialPickGeometry): {
	readonly anchors: Record<string, AnchorRecord>;
	readonly vertices: Record<string, VertexRecord>;
	readonly edges: Record<string, EdgeRecord>;
	readonly wires: Record<string, WireRecord>;
	readonly faces: Record<string, FaceRecord>;
	readonly shells: Record<string, ShellRecord>;
	readonly cells: Record<string, CellRecord>;
	readonly cellComplexes: Record<string, CellComplexRecord>;
	readonly clusters: Record<string, ClusterRecord>;
} {
	if (g instanceof TopologyGraph) {
		return {
			anchors: g.anchors,
			vertices: g.vertices,
			edges: g.edges,
			wires: g.wires,
			faces: g.faces,
			shells: g.shells,
			cells: g.cells,
			cellComplexes: g.cellComplexes,
			clusters: g.clusters,
		};
	}
	return {
		anchors: asRecordBucket((g as TopologyGraphJson & { readonly anchors?: readonly AnchorRecord[] }).anchors),
		vertices: asRecordBucket(g.vertices),
		edges: asRecordBucket(g.edges),
		wires: asRecordBucket(g.wires),
		faces: asRecordBucket(g.faces),
		shells: asRecordBucket(g.shells),
		cells: asRecordBucket(g.cells),
		cellComplexes: asRecordBucket(g.cellComplexes),
		clusters: asRecordBucket(g.clusters),
	};
}

function topologyRecords<T>(records: Record<string, T> | undefined): readonly T[] {
	return records ? Object.values(records) : [];
}

function topologyPointCentroid(points: readonly Vec3[]): Vec3 | null {
	if (points.length === 0) return null;
	const sum = points.reduce(
		(acc, p) => [acc[0] + p[0], acc[1] + p[1], acc[2] + p[2]] as unknown as Vec3,
		[0, 0, 0] as unknown as Vec3,
	);
	return [sum[0] / points.length, sum[1] / points.length, sum[2] / points.length] as unknown as Vec3;
}

function topologyVertexPoint(vertices: Record<string, VertexRecord>, id: string): Vec3 | null {
	return vertices[id]?.position ?? null;
}

function topologyEdgePoints(vertices: Record<string, VertexRecord>, edge: EdgeRecord): readonly Vec3[] {
	return edgeSamplePoints(vertices, edge, 32);
}

function topologyFacePoints(
	vertices: Record<string, VertexRecord>,
	edges: Record<string, EdgeRecord>,
	wires: Record<string, WireRecord>,
	face: FaceRecord,
): readonly Vec3[] {
	const ids = face.wireIds.flatMap((wireId) => wires[wireId]?.edgeIds ?? []);
	const points = ids.flatMap((id) => {
		const edge = edges[id];
		return edge ? topologyEdgePoints(vertices, edge) : [];
	});
	const unique = new Map(points.map((p) => [p.join(","), p]));
	return [...unique.values()];
}

function uniqueTopologyPoints(points: readonly Vec3[]): readonly Vec3[] {
	return [...new Map(points.map((p) => [p.join(","), p])).values()];
}

function topologyWirePoints(vertices: Record<string, VertexRecord>, edges: Record<string, EdgeRecord>, wire: WireRecord): readonly Vec3[] {
	return uniqueTopologyPoints(wire.edgeIds.flatMap((id) => (edges[id] ? topologyEdgePoints(vertices, edges[id]!) : [])));
}

function topologyShellPoints(
	vertices: Record<string, VertexRecord>,
	edges: Record<string, EdgeRecord>,
	wires: Record<string, WireRecord>,
	faces: Record<string, FaceRecord>,
	shell: ShellRecord,
): readonly Vec3[] {
	return uniqueTopologyPoints(
		shell.faceIds.flatMap((id) => (faces[id] ? topologyFacePoints(vertices, edges, wires, faces[id]!) : [])),
	);
}

function topologyCellPoints(
	vertices: Record<string, VertexRecord>,
	edges: Record<string, EdgeRecord>,
	wires: Record<string, WireRecord>,
	faces: Record<string, FaceRecord>,
	shells: Record<string, ShellRecord>,
	cell: CellRecord,
): readonly Vec3[] {
	return uniqueTopologyPoints(
		cell.shellIds.flatMap((id) => (shells[id] ? topologyShellPoints(vertices, edges, wires, faces, shells[id]!) : [])),
	);
}

function topologyCellComplexPoints(
	vertices: Record<string, VertexRecord>,
	edges: Record<string, EdgeRecord>,
	wires: Record<string, WireRecord>,
	faces: Record<string, FaceRecord>,
	shells: Record<string, ShellRecord>,
	cells: Record<string, CellRecord>,
	complex: CellComplexRecord,
): readonly Vec3[] {
	return uniqueTopologyPoints(
		complex.cellIds.flatMap((id) => (cells[id] ? topologyCellPoints(vertices, edges, wires, faces, shells, cells[id]!) : [])),
	);
}

function topologyAllVertexPoints(vertices: Record<string, VertexRecord>): readonly Vec3[] {
	return topologyRecords(vertices).map((vertex) => vertex.position);
}

function topologyEntityPoints(
	buckets: ReturnType<typeof topologyGeometryBuckets>,
	kind: SpatialPickTargetKind,
	id: string,
): readonly Vec3[] {
	if (kind === "anchor") {
		const anchor = buckets.anchors[id];
		return anchor ? [anchor.position] : [];
	}
	if (kind === "vertex") return buckets.vertices[id]?.position ? [buckets.vertices[id]!.position] : [];
	if (kind === "edge" && buckets.edges[id]) return topologyEdgePoints(buckets.vertices, buckets.edges[id]!);
	if (kind === "wire" && buckets.wires[id]) return topologyWirePoints(buckets.vertices, buckets.edges, buckets.wires[id]!);
	if (kind === "face" && buckets.faces[id]) return topologyFacePoints(buckets.vertices, buckets.edges, buckets.wires, buckets.faces[id]!);
	if (kind === "shell" && buckets.shells[id]) return topologyShellPoints(buckets.vertices, buckets.edges, buckets.wires, buckets.faces, buckets.shells[id]!);
	if (kind === "cell" && buckets.cells[id]) return topologyCellPoints(buckets.vertices, buckets.edges, buckets.wires, buckets.faces, buckets.shells, buckets.cells[id]!);
	if (kind === "cellComplex" && buckets.cellComplexes[id]) {
		return topologyCellComplexPoints(
			buckets.vertices,
			buckets.edges,
			buckets.wires,
			buckets.faces,
			buckets.shells,
			buckets.cells,
			buckets.cellComplexes[id]!,
		);
	}
	if (kind === "surface" || kind === "part") return [];
	return [];
}

function topologyWireEdgeSegments(
	vertices: Record<string, VertexRecord>,
	edges: Record<string, EdgeRecord>,
	wire: WireRecord,
): readonly (readonly [Vec3, Vec3])[] {
	const out: (readonly [Vec3, Vec3])[] = [];
	for (const edgeId of wire.edgeIds) {
		const edge = edges[edgeId];
		if (!edge) continue;
		const pts = topologyEdgePoints(vertices, edge);
		if (pts.length >= 2) out.push([pts[0]!, pts[1]!]);
	}
	return out;
}

/** @emoji 📐 Topology wire segments for previews (edges/wires/faces), bbox fallback for aggregates. */
export function topologyEntityWireSegments(
	buckets: ReturnType<typeof topologyGeometryBuckets>,
	kind: TopologyEntityKind,
	id: string,
): readonly (readonly [Vec3, Vec3])[] {
	if (kind === "edge" && buckets.edges[id]) return topologyWireEdgeSegments(buckets.vertices, buckets.edges, { id, edgeIds: [id] });
	if (kind === "wire" && buckets.wires[id]) return topologyWireEdgeSegments(buckets.vertices, buckets.edges, buckets.wires[id]!);
	if (kind === "face" && buckets.faces[id]) {
		const face = buckets.faces[id]!;
		return face.wireIds.flatMap((wireId) => {
			const wire = buckets.wires[wireId];
			return wire ? topologyWireEdgeSegments(buckets.vertices, buckets.edges, wire) : [];
		});
	}
	if (kind === "shell" && buckets.shells[id]) {
		return buckets.shells[id]!.faceIds.flatMap((faceId) => topologyEntityWireSegments(buckets, "face", faceId));
	}
	if (kind === "cell" && buckets.cells[id]) {
		return buckets.cells[id]!.shellIds.flatMap((shellId) => topologyEntityWireSegments(buckets, "shell", shellId));
	}
	if (kind === "cellComplex" && buckets.cellComplexes[id]) {
		return buckets.cellComplexes[id]!.cellIds.flatMap((cellId) => topologyEntityWireSegments(buckets, "cell", cellId));
	}
	const pts = topologyEntityPoints(buckets, kind as SpatialPickTargetKind, id);
	const bb = bboxFromPoints(pts);
	return bb ? bboxWireSegments(bb.min, bb.max) : [];
}

/** @emoji 📐 All B-rep edge segments for factory topology wireframe display. */
export function collectTopologyEdgeSegments(
	buckets: ReturnType<typeof topologyGeometryBuckets>,
): readonly (readonly [Vec3, Vec3])[] {
	const out: (readonly [Vec3, Vec3])[] = [];
	for (const edge of topologyRecords(buckets.edges)) {
		const pts = topologyEdgePoints(buckets.vertices, edge);
		if (pts.length >= 2) out.push([pts[0]!, pts[pts.length - 1]!]);
	}
	return out;
}

function intersectCellAabbs(topo: TopologyGraph, cellIds: readonly string[]): { readonly min: Vec3; readonly max: Vec3 } | null {
	let hit: { readonly min: Vec3; readonly max: Vec3 } | null = null;
	for (const cellId of cellIds) {
		const cell = topo.cells[cellId];
		if (!cell) continue;
		const aabb = topologyCellAabb(topo, cell);
		if (!aabb) continue;
		if (!hit) {
			hit = aabb;
			continue;
		}
		const min: Vec3 = [Math.max(hit.min[0], aabb.min[0]), Math.max(hit.min[1], aabb.min[1]), Math.max(hit.min[2], aabb.min[2])];
		const max: Vec3 = [Math.min(hit.max[0], aabb.max[0]), Math.min(hit.max[1], aabb.max[1]), Math.min(hit.max[2], aabb.max[2])];
		if (min[0] >= max[0] || min[1] >= max[1] || min[2] >= max[2]) return null;
		hit = { min, max };
	}
	return hit;
}

function aabbCornerPoints(min: Vec3, max: Vec3): readonly Vec3[] {
	return [
		[min[0], min[1], min[2]],
		[max[0], min[1], min[2]],
		[max[0], max[1], min[2]],
		[min[0], max[1], min[2]],
		[min[0], min[1], max[2]],
		[max[0], min[1], max[2]],
		[max[0], max[1], max[2]],
		[min[0], max[1], max[2]],
	];
}

function partPickPoints(
	topo: TopologyGraph,
	buckets: ReturnType<typeof topologyGeometryBuckets>,
	part: PartView,
	fallback: readonly Vec3[],
): readonly Vec3[] {
	if (part.overlap === "intersection" && part.sourceCellIds.length >= 2) {
		const inter = intersectCellAabbs(topo, part.sourceCellIds);
		if (inter) return aabbCornerPoints(inter.min, inter.max);
	}
	const fromCells = uniqueTopologyPoints(part.sourceCellIds.flatMap((cellId) => topologyEntityPoints(buckets, "cell", cellId)));
	return fromCells.length ? fromCells : fallback;
}

function createAnalyticSpatialPickTargets(
	buckets: ReturnType<typeof topologyGeometryBuckets>,
	derived: DerivedViewService,
	topo: TopologyGraph,
): readonly SpatialPickTarget[] {
	const targets: SpatialPickTarget[] = [];
	const all = topologyAllVertexPoints(buckets.vertices);
	const allCenter = topologyPointCentroid(all);
	for (const surface of derived.computeSurfaces(topo)) {
		const points = surface.sourceFaceIds.flatMap((faceId) => topologyEntityPoints(buckets, "face", faceId));
		const merged = uniqueTopologyPoints(points);
		const point = topologyPointCentroid(merged);
		if (!point) continue;
		targets.push({
			kind: "surface",
			id: String(surface.id),
			point,
			points: merged.length ? merged : undefined,
			derivedFrom: surface.sourceFaceIds.map((id) => ({ kind: "face" as const, id })),
			exposure: surface.exposure,
		});
	}
	for (const part of derived.computeParts(topo)) {
		const merged = uniqueTopologyPoints(partPickPoints(topo, buckets, part, all));
		const point = topologyPointCentroid(merged) ?? allCenter;
		if (!point) continue;
		targets.push({
			kind: "part",
			id: String(part.id),
			point,
			points: merged.length ? merged : all,
			derivedFrom: part.sourceCellIds.map((id) => ({ kind: "cell" as const, id })),
			overlap: part.overlap,
		});
	}
	return targets;
}

/** @emoji 🧲 Builds renderer-side snap/select targets from optional factory topology geometry. */
export function createSpatialPickTargets(
	geometry: SpatialPickGeometry | null | undefined,
	derived?: DerivedViewService | null,
): readonly SpatialPickTarget[] {
	if (!geometry) return [];
	const buckets = topologyGeometryBuckets(geometry);
	const topo = geometry instanceof TopologyGraph ? geometry : parseTopologyGraphJson(geometry as TopologyGraphJson);
	const targets: SpatialPickTarget[] = [];
	if (topo) {
		for (const anchor of topologyRecords(buckets.anchors)) {
			targets.push({ kind: "anchor", id: anchor.id, point: evaluateAnchorPosition(topo, anchor) });
		}
	}
	for (const vertex of topologyRecords(buckets.vertices)) {
		targets.push({ kind: "vertex", id: vertex.id, point: vertex.position });
	}
	for (const edge of topologyRecords(buckets.edges)) {
		const points = topologyEdgePoints(buckets.vertices, edge);
		const point = topologyPointCentroid(points);
		if (point) targets.push({ kind: "edge", id: edge.id, point, points });
	}
	for (const wire of topologyRecords(buckets.wires)) {
		const points = topologyWirePoints(buckets.vertices, buckets.edges, wire);
		const point = topologyPointCentroid(points);
		if (point) targets.push({ kind: "wire", id: wire.id, point, points });
	}
	for (const face of topologyRecords(buckets.faces)) {
		const points = topologyFacePoints(buckets.vertices, buckets.edges, buckets.wires, face);
		const point = topologyPointCentroid(points);
		if (point) targets.push({ kind: "face", id: face.id, point, points });
	}
	for (const shell of topologyRecords(buckets.shells)) {
		const points = topologyShellPoints(buckets.vertices, buckets.edges, buckets.wires, buckets.faces, shell);
		const point = topologyPointCentroid(points);
		if (point) targets.push({ kind: "shell", id: shell.id, point, points });
	}
	const all = topologyAllVertexPoints(buckets.vertices);
	const allCenter = topologyPointCentroid(all);
	for (const cell of topologyRecords(buckets.cells)) {
		const points = topologyCellPoints(buckets.vertices, buckets.edges, buckets.wires, buckets.faces, buckets.shells, cell);
		const point = topologyPointCentroid(points) ?? allCenter;
		if (point) targets.push({ kind: "cell", id: cell.id, point, points: points.length ? points : all });
	}
	for (const complex of topologyRecords(buckets.cellComplexes)) {
		const points = topologyCellComplexPoints(
			buckets.vertices,
			buckets.edges,
			buckets.wires,
			buckets.faces,
			buckets.shells,
			buckets.cells,
			complex,
		);
		const point = topologyPointCentroid(points) ?? allCenter;
		if (point) targets.push({ kind: "cellComplex", id: complex.id, point, points: points.length ? points : all });
	}
	for (const cluster of topologyRecords(buckets.clusters)) {
		const points = uniqueTopologyPoints(
			cluster.memberIds.flatMap((id) => {
				for (const kind of SPATIAL_PICK_TARGET_KINDS) {
					const hit = topologyEntityPoints(buckets, kind, id);
					if (hit.length) return hit;
				}
				return [];
			}),
		);
		const point = topologyPointCentroid(points) ?? allCenter;
		if (point) targets.push({ kind: "cluster", id: cluster.id, point, points: points.length ? points : all });
	}
	if (derived && topo) targets.push(...createAnalyticSpatialPickTargets(buckets, derived, topo));
	return targets;
}

export function filterSpatialPickTargets(
	targets: readonly SpatialPickTarget[],
	accept: readonly TopologyEntityKind[] = [],
	toggles: SpatialPickKindToggles = {},
): SpatialPickTarget[] {
	const acceptSet = accept.length ? new Set<TopologyEntityKind>(accept) : null;
	return targets.filter((target) => toggles[target.kind] !== false && (!acceptSet || acceptSet.has(target.kind)));
}

function filterSpatialPickTargetsForAnyToggle(
	targets: readonly SpatialPickTarget[],
	...toggleSets: readonly SpatialPickKindToggles[]
): SpatialPickTarget[] {
	return targets.filter((target) => toggleSets.some((toggles) => toggles[target.kind] !== false));
}

/** @emoji 🧲 Creates a statechart event carrying snapped point plus selected topology metadata. */
export function createSpatialPickEvent(
	kind: SpatialPickKind,
	point: Vec3,
	target: SpatialPickTarget | null,
	modifiers: InteractionEvent["modifiers"] = {},
): InteractionEvent {
	return target
		? {
				kind,
				point,
				modifiers,
				snap: { kind: target.kind, id: target.id, point: target.point },
				selection: { kind: target.kind, id: target.id },
			}
		: { kind, point, modifiers };
}
// #endregion 🧲TopologyTargets

// #region 🖼️DisplayPrimitives
function BoxPreviewItem({ item }: { readonly item: DisplayItem }): ReactNode {
	const p = item.params;
	const edgeGeo = useMemo(() => new THREE.EdgesGeometry(new THREE.BoxGeometry(1, 1, 1)), []);
	if (!p) return null;
	const a = readVec3(p.cornerA);
	const b = readVec3(p.cornerB);
	const hRaw = readNumber(p.height);
	if (!a || !b) return null;
	const h = hRaw === null || hRaw <= 0 ? 0.06 : hRaw;
	const { position, scale } = computeBoxPreviewLayout(a, b, h);
	const archived = item.role === "archived";
	return (
		<group position={position} scale={scale}>
			<mesh raycast={raycastNone}>
				<boxGeometry args={[1, 1, 1]} />
				<meshStandardMaterial
					color={archived ? "#5a8c6a" : "#7ab0ff"}
					emissive={archived ? "#0a2818" : "#102a66"}
					emissiveIntensity={archived ? 0.22 : 0.35}
					transparent
					opacity={archived ? 0.38 : 0.52}
					depthWrite={false}
				/>
			</mesh>
			<lineSegments raycast={raycastNone} geometry={edgeGeo}>
				<lineBasicMaterial color={archived ? "#a8d4b8" : "#ffffff"} transparent opacity={archived ? 0.55 : 0.85} />
			</lineSegments>
		</group>
	);
}

function PointItem({ item }: { readonly item: DisplayItem }): ReactNode {
	const pos = readVec3(item.params?.position);
	if (!pos) return null;
	const cursor = item.role === "cursor";
	const r = cursor ? 0.045 : 0.06;
	return (
		<mesh position={pos} raycast={raycastNone}>
			<sphereGeometry args={[r, 16, 16]} />
			<meshStandardMaterial
				color={cursor ? "#66e8ff" : "#ffcc66"}
				emissive={cursor ? "#003844" : "#553300"}
				emissiveIntensity={cursor ? 0.45 : 0.35}
			/>
		</mesh>
	);
}

function LinearHandleItem({ item }: { readonly item: DisplayItem }): ReactNode {
	const p = item.params;
	if (!p) return null;
	const origin = readVec3(p.origin);
	const axis = readVec3(p.axis);
	if (!origin || !axis) return null;
	const ax = axis[0];
	const ay = axis[1];
	const az = axis[2];
	const len = Math.hypot(ax, ay, az) || 1;
	const ux = ax / len;
	const uy = ay / len;
	const uz = az / len;
	const span = 5;
	const x1 = origin[0] + ux * span;
	const y1 = origin[1] + uy * span;
	const z1 = origin[2] + uz * span;
	return (
		<Line
			raycast={raycastNone}
			points={[
				[origin[0], origin[1], origin[2]],
				[x1, y1, z1],
			]}
			color="#ffff88"
			lineWidth={2}
			dashed={false}
		/>
	);
}

function SegmentItem({ item }: { readonly item: DisplayItem }): ReactNode {
	const p = item.params;
	if (!p) return null;
	const a = readVec3(p.from);
	const b = readVec3(p.to);
	if (!a || !b) return null;
	return (
		<Line
			raycast={raycastNone}
			points={[
				[a[0], a[1], a[2]],
				[b[0], b[1], b[2]],
			]}
			color="#88eeff"
			lineWidth={2}
			dashed={false}
		/>
	);
}

function LabelItem({ item }: { readonly item: DisplayItem }): ReactNode {
	const p = item.params;
	if (!p) return null;
	const pos = readVec3(p.position);
	const text = p.text;
	if (!pos || typeof text !== "string") return null;
	return (
		<Text position={pos} fontSize={0.22} color="#f4f4ff" anchorX="left" anchorY="bottom" raycast={raycastNone}>
			{text}
		</Text>
	);
}

function TopologyTargetWireframes({
	geometry,
	targets,
	transform,
	color,
	opacity,
}: {
	readonly geometry: SpatialPickGeometry;
	readonly targets: readonly { readonly kind: TopologyEntityKind; readonly id: string }[];
	readonly transform: (point: Vec3) => Vec3;
	readonly color: string;
	readonly opacity: number;
}): ReactNode {
	const buckets = useMemo(() => topologyGeometryBuckets(geometry), [geometry]);
	const segments = useMemo(() => {
		const out: (readonly [Vec3, Vec3])[] = [];
		for (const target of targets) {
			for (const [a, b] of topologyEntityWireSegments(buckets, target.kind, target.id)) {
				out.push([transform(a), transform(b)]);
			}
		}
		return out;
	}, [buckets, targets, transform]);
	if (!segments.length) return null;
	return (
		<group>
			{segments.map(([a, b], i) => (
				<Line
					key={`${a[0]}-${a[1]}-${a[2]}-${b[0]}-${b[1]}-${b[2]}-${i}`}
					raycast={raycastNone}
					points={[
						[a[0], a[1], a[2]],
						[b[0], b[1], b[2]],
					]}
					color={color}
					lineWidth={2}
					transparent
					opacity={opacity}
				/>
			))}
		</group>
	);
}

function TopologyTargetPreviewMeshes({
	geometry,
	targets,
	transform,
	color,
	opacity,
}: {
	readonly geometry: SpatialPickGeometry;
	readonly targets: readonly { readonly kind: TopologyEntityKind; readonly id: string }[];
	readonly transform: (point: Vec3) => Vec3;
	readonly color: string;
	readonly opacity: number;
}): ReactNode {
	const buckets = useMemo(() => topologyGeometryBuckets(geometry), [geometry]);
	const solids = useMemo(() => {
		const out: { readonly key: string; readonly center: Vec3; readonly size: Vec3 }[] = [];
		for (const target of targets) {
			const pts = topologyEntityPoints(buckets, target.kind as SpatialPickTargetKind, target.id).map(transform);
			if (target.kind === "vertex" && pts[0]) {
				out.push({ key: `${target.kind}:${target.id}:v`, center: pts[0], size: [0.1, 0.1, 0.1] });
				continue;
			}
			if (target.kind === "edge" || target.kind === "wire") continue;
			const bounds = targetBounds(pts);
			if (!bounds) continue;
			out.push({ key: `${target.kind}:${target.id}`, center: bounds.center, size: bounds.size });
		}
		return out;
	}, [buckets, targets, transform]);
	if (!solids.length) return null;
	return (
		<group>
			{solids.map((solid) => (
				<mesh key={solid.key} position={solid.center} scale={solid.size} raycast={raycastNone}>
					<boxGeometry args={[1, 1, 1]} />
					<meshStandardMaterial
						color={color}
						emissive={color}
						emissiveIntensity={0.12}
						transparent
						opacity={opacity}
						depthWrite={false}
						side={THREE.DoubleSide}
					/>
				</mesh>
			))}
		</group>
	);
}

function PreviewItem({
	item,
	geometry,
}: {
	readonly item: DisplayItem;
	readonly geometry?: SpatialPickGeometry | null;
}): ReactNode {
	const p = item.params;
	if (!p) return null;
	const previewKind = typeof p.previewKind === "string" ? p.previewKind : "preview";
	const targets = parseDisplaySelectionTargets(p.targets);
	const transform = useMemo(() => transformPointsForPreviewKind(previewKind, p), [previewKind, p]);
	const points = readVec3Array(p.points);
	const cursor = readVec3(p.cursor);
	const prevPoint = readVec3(p.prevPoint);
	const from = readVec3(p.from) ?? prevPoint;
	const linePoints = points.length ? [...points, ...(cursor ? [cursor] : [])] : from && cursor ? [from, cursor] : [];
	const ghost =
		previewKind === "move-preview" || previewKind === "copy-preview" || previewKind === "mirror-preview";
	const wireColor =
		previewKind === "selected-objects" || previewKind.endsWith("-selection") ? "#ffcc66" : ghost ? "#7ab0ff" : "#88eeff";
	const wireOpacity = ghost ? 0.92 : 0.78;
	const meshColor = ghost ? "#4a6088" : wireColor;
	const meshOpacity = ghost ? 0.28 : 0.42;
	if (geometry && targets.length && previewKindUsesTopologyWireframe(previewKind)) {
		return (
			<group>
				{ghost ? (
					<>
						<TopologyTargetPreviewMeshes
							geometry={geometry}
							targets={targets}
							transform={(pt) => pt}
							color={meshColor}
							opacity={meshOpacity}
						/>
						<TopologyTargetWireframes
							geometry={geometry}
							targets={targets}
							transform={(pt) => pt}
							color="#4a6088"
							opacity={0.35}
						/>
					</>
				) : null}
				<TopologyTargetPreviewMeshes
					geometry={geometry}
					targets={targets}
					transform={transform}
					color={meshColor}
					opacity={meshOpacity}
				/>
				<TopologyTargetWireframes
					geometry={geometry}
					targets={targets}
					transform={transform}
					color={wireColor}
					opacity={wireOpacity}
				/>
				{from ? (
					<mesh position={from} raycast={raycastNone}>
						<sphereGeometry args={[0.05, 12, 12]} />
						<meshStandardMaterial color="#ff9966" emissive="#442200" emissiveIntensity={0.4} />
					</mesh>
				) : null}
				{linePoints.length >= 2 ? (
					<Line raycast={raycastNone} points={linePoints.map((pt) => [pt[0], pt[1], pt[2]])} color="#ffff88" lineWidth={2} />
				) : null}
			</group>
		);
	}
	// #region 🔵CircleArcPreview
	if ((previewKind === "circle-outline" || previewKind === "circle") && points.length >= 1 && cursor) {
		const center = points[0]!;
		const radius = Math.hypot(cursor[0] - center[0], cursor[1] - center[1], cursor[2] - center[2]);
		if (radius > 1e-9) {
			const segments = 64;
			const circlePts: [number, number, number][] = [];
			for (let i = 0; i <= segments; i++) {
				const a = (i / segments) * Math.PI * 2;
				circlePts.push([center[0] + Math.cos(a) * radius, center[1] + Math.sin(a) * radius, center[2]]);
			}
			return (
				<group>
					<Line raycast={raycastNone} points={circlePts} color="#88eeff" lineWidth={2} />
					<Line
						raycast={raycastNone}
						points={[[center[0], center[1], center[2]], [cursor[0], cursor[1], cursor[2]]]}
						color="#ffff88"
						lineWidth={1.5}
						dashed
						dashSize={0.08}
						gapSize={0.06}
					/>
					<mesh position={center} raycast={raycastNone}>
						<sphereGeometry args={[0.04, 10, 10]} />
						<meshStandardMaterial color="#ffcc66" emissive="#553300" emissiveIntensity={0.35} />
					</mesh>
				</group>
			);
		}
	}
	if (previewKind === "arc" && points.length >= 2 && cursor) {
		const center = points[0]!;
		const start = points[1]!;
		const arcEnd = arcEndOnCircle(center, start, cursor);
		const arcPts = arcSamplePoints(center, start, arcEnd, 64);
		if (arcPts.length >= 2) {
			return (
				<group>
					<Line raycast={raycastNone} points={arcPts.map((pt) => [pt[0], pt[1], pt[2]])} color="#88eeff" lineWidth={2} />
					<Line
						raycast={raycastNone}
						points={[[center[0], center[1], center[2]], [start[0], start[1], start[2]]]}
						color="#ffff88"
						lineWidth={1.5}
						dashed
						dashSize={0.08}
						gapSize={0.06}
					/>
					<Line
						raycast={raycastNone}
						points={[[center[0], center[1], center[2]], [arcEnd[0], arcEnd[1], arcEnd[2]]]}
						color="#ffff88"
						lineWidth={1.5}
						dashed
						dashSize={0.08}
						gapSize={0.06}
					/>
					<mesh position={center} raycast={raycastNone}>
						<sphereGeometry args={[0.04, 10, 10]} />
						<meshStandardMaterial color="#ffcc66" emissive="#553300" emissiveIntensity={0.35} />
					</mesh>
					<mesh position={start} raycast={raycastNone}>
						<sphereGeometry args={[0.04, 10, 10]} />
						<meshStandardMaterial color="#ffcc66" emissive="#553300" emissiveIntensity={0.35} />
					</mesh>
					<mesh position={arcEnd} raycast={raycastNone}>
						<sphereGeometry args={[0.04, 10, 10]} />
						<meshStandardMaterial color="#88eeff" emissive="#113344" emissiveIntensity={0.35} />
					</mesh>
				</group>
			);
		}
	}
	// #endregion 🔵CircleArcPreview
	if (previewKind === "interpolated-curve" && linePoints.length >= 2) {
		const splinePoints = linePoints.map((pt) => new THREE.Vector3(pt[0], pt[1], pt[2]));
		const curve = new THREE.CatmullRomCurve3(splinePoints);
		const segments = Math.max(64, splinePoints.length * 16);
		const sampled = curve.getPoints(segments).map((v): [number, number, number] => [v.x, v.y, v.z]);
		const placedCount = cursor ? splinePoints.length - 1 : splinePoints.length;
		return (
			<group>
				<Line raycast={raycastNone} points={sampled} color="#88eeff" lineWidth={2} />
				{splinePoints.slice(0, placedCount).map((v, i) => (
					<mesh key={i} position={[v.x, v.y, v.z]} raycast={raycastNone}>
						<sphereGeometry args={[0.04, 10, 10]} />
						<meshStandardMaterial color="#ffcc66" emissive="#553300" emissiveIntensity={0.35} />
					</mesh>
				))}
			</group>
		);
	}
	return (
		<group>
			{linePoints.length >= 2 ? (
				<Line raycast={raycastNone} points={linePoints.map((pt) => [pt[0], pt[1], pt[2]])} color="#88eeff" lineWidth={2} />
			) : null}
		</group>
	);
}

function EntityHighlightItem({
	item,
	geometry,
}: {
	readonly item: DisplayItem;
	readonly geometry?: SpatialPickGeometry | null;
}): ReactNode {
	const p = item.params;
	if (!p || !geometry) return null;
	const entity = p.entity;
	if (!entity || typeof entity !== "object") return null;
	const kind = (entity as { kind?: unknown }).kind;
	const id = (entity as { id?: unknown }).id;
	if (typeof kind !== "string" || typeof id !== "string") return null;
	return (
		<TopologyTargetWireframes
			geometry={geometry}
			targets={[{ kind: kind as TopologyEntityKind, id }]}
			transform={(pt) => pt}
			color="#ffcc66"
			opacity={0.85}
		/>
	);
}

function DisplayItemNode({
	item,
	geometry,
}: {
	readonly item: DisplayItem;
	readonly geometry?: SpatialPickGeometry | null;
}): ReactNode {
	switch (item.kind) {
		case "box-preview":
			return <BoxPreviewItem item={item} />;
		case "point":
			return <PointItem item={item} />;
		case "linear-handle":
			return <LinearHandleItem item={item} />;
		case "segment":
			return <SegmentItem item={item} />;
		case "label":
			return <LabelItem item={item} />;
		case "preview":
			return <PreviewItem item={item} geometry={geometry} />;
		case "entity-highlight":
			return <EntityHighlightItem item={item} geometry={geometry} />;
		default:
			return null;
	}
}

/** @emoji 🖼️ Maps `DisplayModel.items` to R3F nodes (must live under `<Canvas>`). */
export function InteractionDisplay({
	model,
	geometry,
}: {
	readonly model: DisplayModel;
	readonly geometry?: SpatialPickGeometry | null;
}): ReactNode {
	return (
		<group>
			{model.items.map((item) => (
				<group key={item.id}>
					<DisplayItemNode item={item} geometry={geometry} />
				</group>
			))}
		</group>
	);
}
// #endregion 🖼️DisplayPrimitives

// #region 🖱️Interaction
function pointerModifiers(event: ThreeEvent<PointerEvent>) {
	return {
		alt: event.altKey,
		ctrl: event.ctrlKey,
		meta: event.metaKey,
		shift: event.shiftKey,
	};
}

/** @emoji 🖱️ Ground hit-test on the **XY** working plane at fixed world **Z** (= spatial footprint plane; factory height is world Z). */
export interface GroundPickPlaneProps {
	readonly planeZ?: number;
	readonly enabled?: boolean;
	readonly onPick?: (point: Vec3) => void;
	readonly onContextPick?: (point: Vec3) => void;
	readonly onPointerMove?: (point: Vec3) => void;
	readonly pointerMoveEnabled?: boolean;
}

export function GroundPickPlane({
	planeZ = 0,
	enabled = true,
	onPick,
	onContextPick,
	onPointerMove,
	pointerMoveEnabled,
}: GroundPickPlaneProps): ReactNode {
	const moveOn = pointerMoveEnabled ?? Boolean(onPointerMove);
	const onPointerDown = (e: ThreeEvent<PointerEvent>) => {
		if (!enabled || !onPick) return;
		e.stopPropagation();
		const p = e.point;
		onPick([p.x, p.y, planeZ] as unknown as Vec3);
	};
	const onContextMenu = (e: ThreeEvent<MouseEvent>) => {
		if (!enabled || !onContextPick) return;
		e.stopPropagation();
		const p = e.point;
		onContextPick([p.x, p.y, planeZ] as unknown as Vec3);
	};
	const onPointerMoveH = (e: ThreeEvent<PointerEvent>) => {
		if (!moveOn || !onPointerMove) return;
		e.stopPropagation();
		const p = e.point;
		onPointerMove([p.x, p.y, planeZ] as unknown as Vec3);
	};
	return (
		<mesh position={[0, 0, planeZ]} onPointerDown={onPointerDown} onContextMenu={onContextMenu} onPointerMove={onPointerMoveH}>
			<planeGeometry args={[120, 120]} />
			<meshBasicMaterial transparent opacity={0.18} color="#7a9dff" side={THREE.DoubleSide} />
		</mesh>
	);
}

function vec3FromSnapshotContext(ctx: Record<string, unknown>, key: string): Vec3 | null {
	return readVec3(ctx[key]);
}

/** @emoji 🖱️ YZ wall at the second corner so `pointer.move` changes world Z (factory height uses |Δz|). */
function HeightDragSurface({
	origin,
	corner,
	enabled,
	onPointerMove,
}: {
	readonly origin: Vec3;
	readonly corner: Vec3;
	readonly enabled: boolean;
	readonly onPointerMove?: (point: Vec3) => void;
}): ReactNode {
	const z0 = origin[2];
	const zSpan = 10;
	const zMid = z0 + zSpan / 2;
	const ySpan = 6;
	const onMove = (e: ThreeEvent<PointerEvent>) => {
		if (!enabled || !onPointerMove) return;
		e.stopPropagation();
		const p = e.point;
		onPointerMove([p.x, p.y, p.z] as unknown as Vec3);
	};
	const xPlane = corner[0] + 0.06;
	return (
		<mesh
			position={[xPlane, corner[1], zMid]}
			rotation={[0, Math.PI / 2, 0]}
			onPointerMove={onMove}
			renderOrder={2}
		>
			<planeGeometry args={[zSpan, ySpan]} />
			<meshStandardMaterial
				transparent
				opacity={0.38}
				color="#3ecf9f"
				emissive="#0a3020"
				emissiveIntensity={0.25}
				roughness={0.88}
				metalness={0.08}
				depthWrite={false}
				side={THREE.DoubleSide}
			/>
		</mesh>
	);
}

/** @emoji 🖱️ Z-aligned rod at `origin` so `pointer.move` drives peak height without XY drift. */
function VerticalZDragRod({
	origin,
	enabled,
	onPointerMove,
}: {
	readonly origin: Vec3;
	readonly enabled: boolean;
	readonly onPointerMove?: (point: Vec3) => void;
}): ReactNode {
	const h = 22;
	const onMove = (e: ThreeEvent<PointerEvent>) => {
		if (!enabled || !onPointerMove) return;
		e.stopPropagation();
		const p = e.point;
		onPointerMove([p.x, p.y, p.z] as unknown as Vec3);
	};
	return (
		<mesh
			position={[origin[0], origin[1], origin[2] + h / 2]}
			rotation={[Math.PI / 2, 0, 0]}
			onPointerMove={onMove}
			renderOrder={3}
		>
			<cylinderGeometry args={[0.14, 0.14, h, 10]} />
			<meshStandardMaterial
				transparent
				opacity={0.14}
				color="#55aaff"
				depthWrite={false}
				side={THREE.DoubleSide}
			/>
		</mesh>
	);
}

/** @emoji 🎮 Maps R3F pointer events to `InteractionEvent` envelopes (point + modifiers). */
export function createR3FInteractionAdapter() {
	const toPoint = (event: ThreeEvent<PointerEvent>): Vec3 => [event.point.x, event.point.y, event.point.z];
	return {
		pointerMove: (event: ThreeEvent<PointerEvent>): InteractionEvent => ({
			kind: "pointer.move",
			point: toPoint(event),
			modifiers: pointerModifiers(event),
		}),
		pointerDown: (event: ThreeEvent<PointerEvent>): InteractionEvent => ({
			kind: "pointer.down",
			point: toPoint(event),
			modifiers: pointerModifiers(event),
		}),
	};
}
// #endregion 🖱️Interaction

// #region 🧲TopologyInteraction
function targetBounds(points: readonly Vec3[]): { readonly center: Vec3; readonly size: Vec3 } | null {
	if (points.length === 0) return null;
	const min = points.reduce(
		(acc, p) => [Math.min(acc[0], p[0]), Math.min(acc[1], p[1]), Math.min(acc[2], p[2])] as unknown as Vec3,
		points[0]!,
	);
	const max = points.reduce(
		(acc, p) => [Math.max(acc[0], p[0]), Math.max(acc[1], p[1]), Math.max(acc[2], p[2])] as unknown as Vec3,
		points[0]!,
	);
	return {
		center: [(min[0] + max[0]) / 2, (min[1] + max[1]) / 2, (min[2] + max[2]) / 2] as unknown as Vec3,
		size: [
			Math.max(max[0] - min[0], 0.08),
			Math.max(max[1] - min[1], 0.08),
			Math.max(max[2] - min[2], 0.08),
		] as unknown as Vec3,
	};
}

const spatialPickPriority: Record<SpatialPickTargetKind, number> = {
	anchor: 0,
	vertex: 0,
	edge: 1,
	wire: 2,
	face: 3,
	surface: 4,
	shell: 5,
	cell: 6,
	part: 7,
	cellComplex: 8,
	cluster: 9,
};

function targetRayScore(ray: THREE.Ray, target: SpatialPickTarget): number | null {
	const points = target.points?.length ? target.points : [target.point];
	const box = new THREE.Box3();
	for (const point of points) box.expandByPoint(new THREE.Vector3(point[0], point[1], point[2]));
	box.expandByScalar(target.kind === "vertex" ? 0.12 : 0.08);
	const hit = ray.intersectBox(box, new THREE.Vector3());
	if (!hit) return null;
	return ray.origin.distanceTo(hit) + spatialPickPriority[target.kind] * 1e-4;
}

function spatialPickTargetsFromRay(
	ray: THREE.Ray,
	targets: readonly SpatialPickTarget[],
	selectionAccept: readonly TopologyEntityKind[],
	kindToggles: SpatialPickKindToggles,
): SpatialPickTarget[] {
	return filterSpatialPickTargets(targets, selectionAccept, kindToggles)
		.map((target) => ({ target, score: targetRayScore(ray, target) }))
		.filter((hit): hit is { readonly target: SpatialPickTarget; readonly score: number } => hit.score !== null)
		.sort((a, b) => a.score - b.score)
		.map((hit) => hit.target);
}

function targetStyle(target: SpatialPickTarget, hovered: boolean, selected: boolean): { color: string; emissive: string; opacity: number; lineWidth: number } {
	if (selected) return { color: "#ff77bb", emissive: "#551233", opacity: target.kind === "vertex" || target.kind === "anchor" ? 1 : 0.34, lineWidth: 9 };
	if (hovered) return { color: "#66e8ff", emissive: "#003844", opacity: target.kind === "vertex" || target.kind === "anchor" ? 1 : 0.28, lineWidth: 8 };
	if (target.kind === "surface") {
		const internal = target.exposure === "internal";
		return {
			color: internal ? "#66bbff" : "#e8c46a",
			emissive: internal ? "#204060" : "#5a4000",
			opacity: internal ? 0.38 : 0.32,
			lineWidth: 7,
		};
	}
	if (target.kind === "part") {
		if (target.overlap === "intersection") return { color: "#ff66cc", emissive: "#660033", opacity: 0.42, lineWidth: 8 };
		if (target.overlap === "difference") return { color: "#ff9944", emissive: "#663300", opacity: 0.38, lineWidth: 8 };
		return { color: "#88dd88", emissive: "#204420", opacity: 0.34, lineWidth: 7 };
	}
	if (target.kind === "anchor") return { color: "#9cffc8", emissive: "#1e4d35", opacity: 1, lineWidth: 5 };
	if (target.kind === "vertex") return { color: "#ffdf7a", emissive: "#4a3000", opacity: 1, lineWidth: 5 };
	if (target.kind === "edge" || target.kind === "wire") return { color: "#ffd166", emissive: "#4a3000", opacity: 0.8, lineWidth: 5 };
	return { color: "#f6c85f", emissive: "#332100", opacity: 0.16, lineWidth: 5 };
}

function spatialSelectionTarget(target: SpatialPickTarget) {
	if (target.kind === "surface" || target.kind === "part") {
		return {
			kind: target.kind,
			id: target.id,
			editable: false,
			derivedFrom: target.derivedFrom ?? [],
		};
	}
	return { kind: target.kind, id: target.id, editable: true };
}

function SpatialSelectionRayCatcher({
	targets,
	selectionAccept,
	selectionKindToggles,
	onSelectionRequest,
}: {
	readonly targets: readonly SpatialPickTarget[];
	readonly selectionAccept: readonly TopologyEntityKind[];
	readonly selectionKindToggles: SpatialPickKindToggles;
	readonly onSelectionRequest?: (request: SpatialSelectionRequest) => void;
}): ReactNode {
	if (selectionAccept.length === 0 || !onSelectionRequest) return null;
	const onPointerDown = (e: ThreeEvent<PointerEvent>) => {
		const candidates = spatialPickTargetsFromRay(e.ray, targets, selectionAccept, selectionKindToggles);
		if (candidates.length === 0) return;
		e.stopPropagation();
		const native = e.nativeEvent;
		onSelectionRequest({
			targets: candidates,
			point: [e.point.x, e.point.y, e.point.z],
			client: { x: native.clientX, y: native.clientY },
			modifiers: pointerModifiers(e),
		});
	};
	return (
		<mesh onPointerDown={onPointerDown} renderOrder={-10}>
			<sphereGeometry args={[80, 16, 16]} />
			<meshBasicMaterial transparent opacity={0} depthWrite={false} side={THREE.BackSide} />
		</mesh>
	);
}

function SpatialPickTargetNode({
	target,
	targets,
	onInteractionEvent,
	onPick,
	onPointerMove,
	onSelectionRequest,
	onHoverTarget,
	pointerMoveEnabled,
	selectionAccept,
	selectionKindToggles,
	hoverKindToggles,
	hoveredTargetKey,
	selectedTargetKey,
}: {
	readonly target: SpatialPickTarget;
	readonly targets: readonly SpatialPickTarget[];
	readonly onInteractionEvent?: (event: InteractionEvent) => void;
	readonly onPick?: (point: Vec3, event: InteractionEvent) => void;
	readonly onPointerMove?: (point: Vec3, event: InteractionEvent) => void;
	readonly onSelectionRequest?: (request: SpatialSelectionRequest) => void;
	readonly onHoverTarget?: (target: SpatialPickTarget | null) => void;
	readonly pointerMoveEnabled: boolean;
	readonly selectionAccept: readonly TopologyEntityKind[];
	readonly selectionKindToggles: SpatialPickKindToggles;
	readonly hoverKindToggles: SpatialPickKindToggles;
	readonly hoveredTargetKey?: string | null;
	readonly selectedTargetKey?: string | null;
}): ReactNode {
	const targetKey = spatialPickTargetKey(target);
	const hovered = hoveredTargetKey === targetKey;
	const selected = selectedTargetKey === targetKey;
	const style = targetStyle(target, hovered, selected);
	const emit = (kind: SpatialPickKind, e: ThreeEvent<PointerEvent>) => {
		e.stopPropagation();
		const event = createSpatialPickEvent(kind, [e.point.x, e.point.y, e.point.z], target, pointerModifiers(e));
		onInteractionEvent?.(event);
		if (kind === "pointer.down") onPick?.(target.point, event);
		if (kind === "pointer.move" && pointerMoveEnabled) onPointerMove?.(target.point, event);
	};
	const onPointerDown = (e: ThreeEvent<PointerEvent>) => {
		e.stopPropagation();
		const candidates = spatialPickTargetsFromRay(e.ray, targets, selectionAccept, selectionKindToggles);
		if (selectionAccept.length > 0 && candidates.length > 0 && onSelectionRequest) {
			const native = e.nativeEvent;
			onSelectionRequest({
				targets: candidates,
				point: [e.point.x, e.point.y, e.point.z],
				client: { x: native.clientX, y: native.clientY },
				modifiers: pointerModifiers(e),
			});
			return;
		}
		emit("pointer.down", e);
	};
	const onPointerMoveH = (e: ThreeEvent<PointerEvent>) => {
		onHoverTarget?.(hoverKindToggles[target.kind] !== false ? target : null);
		if (!pointerMoveEnabled) return;
		emit("pointer.move", e);
	};
	const onPointerOut = () => onHoverTarget?.(null);
	const userData = { spatialPickKey: targetKey };
	if (target.kind === "vertex" || target.kind === "anchor") {
		return (
			<mesh
				position={target.point}
				userData={userData}
				onPointerDown={onPointerDown}
				onPointerMove={onPointerMoveH}
				onPointerOut={onPointerOut}
				renderOrder={4}
			>
				<sphereGeometry args={[selected || hovered ? 0.12 : 0.085, 16, 16]} />
				<meshStandardMaterial color={style.color} emissive={style.emissive} emissiveIntensity={0.45} />
			</mesh>
		);
	}
	if (target.points && target.points.length >= 2 && (target.kind === "edge" || target.kind === "wire")) {
		return (
			<Line
				userData={userData}
				points={target.points.map((p) => [p[0], p[1], p[2]])}
				color={style.color}
				lineWidth={style.lineWidth}
				onPointerDown={onPointerDown}
				onPointerMove={onPointerMoveH}
				onPointerOut={onPointerOut}
			/>
		);
	}
	const bounds = target.points ? targetBounds(target.points) : null;
	if (!bounds) return null;
	return (
		<mesh
			position={bounds.center}
			scale={bounds.size}
			userData={userData}
			onPointerDown={onPointerDown}
			onPointerMove={onPointerMoveH}
			onPointerOut={onPointerOut}
			renderOrder={1}
		>
			<boxGeometry args={[1, 1, 1]} />
			<meshStandardMaterial
				color={style.color}
				emissive={style.emissive}
				emissiveIntensity={hovered || selected ? 0.35 : 0.08}
				transparent
				opacity={style.opacity}
				depthWrite={false}
				side={THREE.DoubleSide}
			/>
		</mesh>
	);
}

/** @emoji 🧵 Draws all topology edges for imported factory geometry. */
function TopologyFactoryWireframeLayer({ geometry }: { readonly geometry?: SpatialPickGeometry | null }): ReactNode {
	const segments = useMemo(() => {
		if (!geometry) return [] as readonly (readonly [Vec3, Vec3])[];
		return collectTopologyEdgeSegments(topologyGeometryBuckets(geometry));
	}, [geometry]);
	if (!segments.length) return null;
	return (
		<group renderOrder={0}>
			{segments.map(([a, b], i) => (
				<Line
					key={`factory-edge-${i}`}
					raycast={raycastNone}
					points={[
						[a[0], a[1], a[2]],
						[b[0], b[1], b[2]],
					]}
					color="#b8c8e8"
					lineWidth={1.5}
					transparent
					opacity={0.72}
				/>
			))}
		</group>
	);
}

/** @emoji 🧲 Renders optional factory geometry as pickable snap/select targets. */
export function SpatialPickGeometryLayer({
	geometry,
	viewKind = "raw",
	derived,
	derivedRevision = 0,
	onInteractionEvent,
	onPick,
	onPointerMove,
	onSelectionRequest,
	onHoverTarget,
	pointerMoveEnabled = false,
	selectionAccept = [],
	selectionKindToggles = {},
	hoverKindToggles = {},
	hoveredTargetKey,
	selectedTargetKey,
}: {
	readonly geometry?: SpatialPickGeometry | null;
	readonly viewKind?: SpatialPickViewKind;
	readonly derived?: DerivedViewService | null;
	readonly derivedRevision?: number;
	readonly onInteractionEvent?: (event: InteractionEvent) => void;
	readonly onPick?: (point: Vec3, event: InteractionEvent) => void;
	readonly onPointerMove?: (point: Vec3, event: InteractionEvent) => void;
	readonly onSelectionRequest?: (request: SpatialSelectionRequest) => void;
	readonly onHoverTarget?: (target: SpatialPickTarget | null) => void;
	readonly pointerMoveEnabled?: boolean;
	readonly selectionAccept?: readonly TopologyEntityKind[];
	readonly selectionKindToggles?: SpatialPickKindToggles;
	readonly hoverKindToggles?: SpatialPickKindToggles;
	readonly hoveredTargetKey?: string | null;
	readonly selectedTargetKey?: string | null;
}): ReactNode {
	const topoRevision =
		geometry && typeof geometry === "object" && "revision" in geometry
			? Number((geometry as { revision?: unknown }).revision)
			: 0;
	const targets = useMemo(() => createSpatialPickTargets(geometry, derived), [geometry, topoRevision, derived, derivedRevision]);
	const viewTargets = useMemo(() => filterSpatialPickTargetsByView(targets, viewKind), [targets, viewKind]);
	const enabledTargets = useMemo(
		() => filterSpatialPickTargetsForAnyToggle(viewTargets, selectionKindToggles, hoverKindToggles),
		[viewTargets, selectionKindToggles, hoverKindToggles],
	);
	return (
		<group>
			<SpatialSelectionRayCatcher
				targets={enabledTargets}
				selectionAccept={selectionAccept}
				selectionKindToggles={selectionKindToggles}
				onSelectionRequest={onSelectionRequest}
			/>
			{enabledTargets.map((target) => (
				<SpatialPickTargetNode
					key={`${target.kind}:${target.id}`}
					target={target}
					targets={enabledTargets}
					onInteractionEvent={onInteractionEvent}
					onPick={onPick}
					onPointerMove={onPointerMove}
					onSelectionRequest={onSelectionRequest}
					onHoverTarget={onHoverTarget}
					pointerMoveEnabled={pointerMoveEnabled}
					selectionAccept={selectionAccept}
					selectionKindToggles={selectionKindToggles}
					hoverKindToggles={hoverKindToggles}
					hoveredTargetKey={hoveredTargetKey}
					selectedTargetKey={selectedTargetKey}
				/>
			))}
		</group>
	);
}
// #endregion 🧲TopologyInteraction

// #region 🧊CommittedMesh
function TessellatedCommitMesh({ mesh: preview }: { readonly mesh: MeshPreview }): ReactNode {
	const geom = useMemo(() => {
		const g = new THREE.BufferGeometry();
		g.setAttribute("position", new THREE.BufferAttribute(preview.positions, 3));
		g.setIndex(new THREE.Uint32BufferAttribute(preview.indices, 1));
		if (preview.normals && preview.normals.length > 0) {
			g.setAttribute("normal", new THREE.BufferAttribute(preview.normals, 3));
		} else {
			g.computeVertexNormals();
		}
		return g;
	}, [preview]);
	return (
		<mesh geometry={geom} raycast={raycastNone}>
			<meshStandardMaterial color="#9ad1ff" metalness={0.15} roughness={0.45} flatShading={false} />
		</mesh>
	);
}
// #endregion 🧊CommittedMesh

// #region 🪝Hooks
/** @emoji 🪝 Memoized `createInteractionRuntime` for React hosts. */
export function useInteractionRuntime(spec: InteractionSpec, opts: InteractionRuntimeOptions): InteractionRuntime {
	return useMemo(() => createInteractionRuntime(spec, opts), [spec, opts]);
}

/** @emoji 🪝 Subscribes to `InteractionRuntime` revision updates for React hosts. */
export function useInteractionSnapshot(rt: InteractionRuntime): InteractionSnapshot {
	return useSyncExternalStore(
		(cb) => rt.subscribe(cb),
		() => rt.getSnapshot(),
		() => rt.getSnapshot(),
	);
}
// #endregion 🪝Hooks

// #region 🪩Canvas
export interface InteractionCanvasProps {
	readonly children: ReactNode;
}

/** @emoji 🪩 Root `<Canvas>` configuration for factory viewports. */
export function InteractionCanvas({ children }: InteractionCanvasProps): ReactNode {
	return (
		<Canvas style={{ height: "100%", width: "100%" }} camera={{ up: [0, 0, 1], position: [10, 10, 8], fov: 45 }}>
			<color attach="background" args={["#080810"]} />
			{children}
		</Canvas>
	);
}

export interface InteractionSpatialViewProps {
	readonly snapshot: InteractionSnapshot;
	readonly onGroundPick?: (point: Vec3, event: InteractionEvent) => void;
	/** @emoji 🖱️ `pointer.move` hits ground (XY at fixed Z); height slab passes full 3D. */
	readonly onScenePointerMove?: (point: Vec3, event: InteractionEvent) => void;
	readonly onInteractionEvent?: (event: InteractionEvent) => void;
	readonly pickEnabled?: boolean;
	readonly committedMesh?: MeshPreview | null;
	readonly geometry?: SpatialPickGeometry | null;
	readonly pickViewKind?: SpatialPickViewKind;
	readonly derived?: DerivedViewService | null;
	readonly derivedRevision?: number;
	/** @emoji 🖼️ When set, drives `InteractionDisplay` instead of `snapshot.display` (e.g. merged archived footprints). */
	readonly displayModel?: DisplayModel;
	readonly selectionAccept?: readonly TopologyEntityKind[];
	readonly selectionKindToggles?: SpatialPickKindToggles;
	readonly hoverKindToggles?: SpatialPickKindToggles;
	readonly hoveredTargetKey?: string | null;
	readonly selectedTargetKey?: string | null;
	readonly onSelectionRequest?: (request: SpatialSelectionRequest) => void;
	readonly onHoverTarget?: (target: SpatialPickTarget | null) => void;
}

/** @emoji 🪩 Lights, orbit controls, ground picking, factory overlays, optional committed mesh. */
export function InteractionSpatialView({
	snapshot,
	onGroundPick,
	onScenePointerMove,
	onInteractionEvent,
	pickEnabled = true,
	committedMesh,
	geometry,
	pickViewKind = "raw",
	derived,
	derivedRevision = 0,
	displayModel,
	selectionAccept = [],
	selectionKindToggles = {},
	hoverKindToggles = {},
	hoveredTargetKey,
	selectedTargetKey,
	onSelectionRequest,
	onHoverTarget,
}: InteractionSpatialViewProps): ReactNode {
	const hostPickGate = pickEnabled !== false;
	const gridHelper = useMemo(() => {
		const g = new THREE.GridHelper(40, 40, 0x3a3a55, 0x1c1c28);
		g.rotation.x = Math.PI / 2;
		g.position.set(0, 0, 0.002);
		return g;
	}, []);
	const ctx = snapshot.context;
	const origin = vec3FromSnapshotContext(ctx, "origin");
	const corner = vec3FromSnapshotContext(ctx, "corner");
	const si = snapshot.spatialInteraction;
	const groundMoveOn =
		si.spatialGroundPick && si.groundPointerMoveStates.includes(snapshot.state) && Boolean(onScenePointerMove);
	const heightMoveOn =
		si.spatialGroundPick &&
		si.heightDragStates.includes(snapshot.state) &&
		Boolean(onScenePointerMove) &&
		origin !== null &&
		corner !== null;
	const zRodMoveOn =
		si.spatialGroundPick &&
		si.verticalRodStates.includes(snapshot.state) &&
		Boolean(onScenePointerMove) &&
		origin !== null;
	const selectionPickOn = selectionAccept.length > 0;
	const pickPlaneEnabled =
		hostPickGate && si.spatialGroundPick && !selectionPickOn && !si.pickDisabledStates.includes(snapshot.state);
	const onGroundPickEvent = (point: Vec3) => {
		const event = createSpatialPickEvent("pointer.down", point, null);
		onInteractionEvent?.(event);
		onGroundPick?.(point, event);
	};
	const onGroundContextEvent = (point: Vec3) => {
		onInteractionEvent?.({ kind: "contextmenu", point, modifiers: {} });
	};
	const onScenePointerMoveEvent = (point: Vec3) => {
		const event = createSpatialPickEvent("pointer.move", point, null);
		onInteractionEvent?.(event);
		onScenePointerMove?.(point, event);
	};
	return (
		<>
			<ambientLight intensity={0.45} />
			<directionalLight position={[12, 18, 10]} intensity={1.1} />
			<OrbitControls
				makeDefault
				mouseButtons={{
					LEFT: -1 as unknown as MOUSE,
					MIDDLE: MOUSE.DOLLY,
					RIGHT: MOUSE.ROTATE,
				}}
			/>
			<primitive object={gridHelper} />
			<GroundPickPlane
				enabled={pickPlaneEnabled}
				onPick={onGroundPickEvent}
				onContextPick={onGroundContextEvent}
				onPointerMove={onScenePointerMoveEvent}
				pointerMoveEnabled={groundMoveOn}
			/>
			<TopologyFactoryWireframeLayer geometry={geometry} />
			<SpatialPickGeometryLayer
				geometry={geometry}
				viewKind={pickViewKind}
				derived={derived}
				derivedRevision={derivedRevision}
				onInteractionEvent={onInteractionEvent}
				onPick={undefined}
				onPointerMove={onScenePointerMove}
				onSelectionRequest={onSelectionRequest}
				onHoverTarget={onHoverTarget}
				pointerMoveEnabled={groundMoveOn || heightMoveOn || zRodMoveOn}
				selectionAccept={selectionAccept}
				selectionKindToggles={selectionKindToggles}
				hoverKindToggles={hoverKindToggles}
				hoveredTargetKey={hoveredTargetKey}
				selectedTargetKey={selectedTargetKey}
			/>
			{heightMoveOn && origin && corner ? (
				<HeightDragSurface
					origin={origin}
					corner={corner}
					enabled={heightMoveOn}
					onPointerMove={onScenePointerMoveEvent}
				/>
			) : null}
			{zRodMoveOn && origin ? (
				<VerticalZDragRod origin={origin} enabled={zRodMoveOn} onPointerMove={onScenePointerMoveEvent} />
			) : null}
			<InteractionDisplay geometry={geometry} model={displayModel ?? snapshot.display} />
			{committedMesh ? <TessellatedCommitMesh mesh={committedMesh} /> : null}
		</>
	);
}
// #endregion 🪩Canvas

// #region 🪩Repl
type ReplSuggestKind = "interaction" | "transition";

interface ReplSuggestion {
	readonly kind: ReplSuggestKind;
	readonly key: string;
	readonly label: string;
	readonly detail: string;
	readonly transition?: InteractionKeybindRow;
	readonly interactionId?: string;
	readonly onRun: () => void;
}

function replFirstWireId(topo: TopologyGraph): string | null {
	const ks = Object.keys(topo.wires);
	return ks.length ? topo.wires[ks[0]!]!.id : null;
}

function replFirstFaceId(topo: TopologyGraph): string | null {
	const ks = Object.keys(topo.faces);
	return ks.length ? topo.faces[ks[0]!]!.id : null;
}

function replBuildDispatchEvent(
	row: InteractionKeybindRow,
	opts: { readonly interactionId: string; readonly topo: TopologyGraph },
): InteractionEvent | null {
	const { interactionId, topo } = opts;
	if (row.eventKind === "set.height" || row.eventKind === "set.distance" || row.eventKind === "set.footprint") return null;
	if (row.eventKind === "selection.changed") {
		if (interactionId === "feature.extrudeWire") {
			const wid = replFirstWireId(topo);
			if (!wid) return null;
			return { kind: "selection.changed", targets: [{ kind: "wire", id: wid, editable: true }], modifiers: {} };
		}
		if (interactionId === "feature.offsetSurface") {
			const fid = replFirstFaceId(topo);
			if (!fid) return null;
			return { kind: "selection.changed", targets: [{ kind: "face", id: fid, editable: true }], modifiers: {} };
		}
		return null;
	}
	return { kind: row.eventKind, modifiers: {} };
}

function replTryParseValueInteraction(line: string, spec: InteractionSpec, state: string): InteractionEvent | null {
	const t = line.trim();
	const m = t.match(/^(\S+)\s+(.+)$/);
	if (!m) return null;
	const head = m[1]!.toLowerCase();
	const tail = m[2]!.trim();
	const rows = listKeyedInteractionTransitions(spec, state);
	for (const row of rows) {
		if (row.eventKind === "set.height") {
			if (head !== row.key.toLowerCase() && head !== "height") continue;
			const v = Number(tail);
			if (!Number.isFinite(v) || v <= 0) return null;
			return { kind: "set.height", value: v, modifiers: {} };
		}
		if (row.eventKind === "set.distance") {
			if (head !== row.key.toLowerCase() && head !== "dist" && head !== "distance") continue;
			const v = Number(tail);
			if (!Number.isFinite(v)) return null;
			return { kind: "set.distance", value: v, modifiers: {} };
		}
		if (row.eventKind === "set.footprint") {
			if (head !== row.key.toLowerCase() && head !== "footprint" && head !== "lw") continue;
			const parts = tail.split(/\s+/);
			const L = Number(parts[0]);
			const W = Number(parts[1]);
			if (!Number.isFinite(L) || !Number.isFinite(W)) return null;
			return { kind: "set.footprint", value: { length: L, width: W }, modifiers: {} };
		}
		if (row.eventKind.startsWith("set.")) {
			const alias = row.eventKind.slice("set.".length).toLowerCase();
			if (head !== row.key.toLowerCase() && head !== alias && head !== "number" && head !== "n") continue;
			const v = Number(tail);
			if (!Number.isFinite(v)) return null;
			return { kind: row.eventKind, value: v, modifiers: {} };
		}
	}
	return null;
}

function replSuggestionHaystack(s: ReplSuggestion): string {
	return `${s.key} ${s.label} ${s.detail}`.toLowerCase();
}

function replRankScore(query: string, s: ReplSuggestion): number {
	const ql = query.trim().toLowerCase();
	if (!ql) return -1;
	const key = s.key.toLowerCase();
	const label = s.label.toLowerCase();
	const detail = s.detail.toLowerCase();
	if (key.startsWith(ql)) return 4000 - key.length;
	if (label.startsWith(ql)) return 3000 - label.length;
	if (detail.startsWith(ql)) return 2000 - detail.length;
	if (replSuggestionHaystack(s).includes(ql)) return 1000;
	return -1;
}

export function replFilterSuggestions(query: string, all: readonly ReplSuggestion[]): ReplSuggestion[] {
	const q = query.trim();
	if (!q) return [];
	return all
		.map((s) => ({ s, score: replRankScore(q, s) }))
		.filter((row) => row.score >= 0)
		.sort((a, b) => b.score - a.score)
		.map((row) => row.s);
}

/** @emoji ⌨️ Inline completion suffix for the active suggestion (longest prefix match on key, label, or detail). */
export function replCompletionSuffix(query: string, suggestion: ReplSuggestion | undefined): string {
	if (!query.trim() || !suggestion) return "";
	const q = query;
	const ql = q.toLowerCase();
	let best = "";
	for (const text of [suggestion.label, suggestion.detail, suggestion.key]) {
		if (!text.toLowerCase().startsWith(ql)) continue;
		const suffix = text.slice(q.length);
		if (suffix.length > best.length) best = suffix;
	}
	return best;
}

/** @emoji ⌨️ First non-empty inline completion suffix across ranked matches. */
export function replActiveCompletionSuffix(query: string, matches: readonly ReplSuggestion[], index: number): string {
	if (!query.trim() || !matches.length) return "";
	const order = [matches[Math.min(index, matches.length - 1)]!, ...matches];
	const seen = new Set<ReplSuggestion>();
	for (const s of order) {
		if (seen.has(s)) continue;
		seen.add(s);
		const suffix = replCompletionSuffix(query, s);
		if (suffix) return suffix;
	}
	return "";
}

export function replPaletteRows(cmdLine: string, all: readonly ReplSuggestion[]): ReplSuggestion[] {
	return replFilterSuggestions(cmdLine, all);
}

function replIsQueryTypingTarget(t: EventTarget | null): boolean {
	return t instanceof HTMLTextAreaElement;
}

function replShouldRepeatInteractionOnSpace(
	event: {
		readonly key: string;
		readonly ctrlKey: boolean;
		readonly metaKey: boolean;
		readonly altKey: boolean;
		readonly defaultPrevented: boolean;
		readonly isComposing: boolean;
		readonly target: EventTarget | null;
	},
	state: {
		readonly interactionActive: boolean;
		readonly cmdTarget: EventTarget | null;
	},
): boolean {
	if (event.defaultPrevented || event.isComposing || state.interactionActive) return false;
	if (event.key !== " " || event.ctrlKey || event.metaKey || event.altKey) return false;
	if (replIsQueryTypingTarget(event.target)) return false;
	return event.target !== state.cmdTarget;
}

function replEscapeAction(state: {
	readonly interactionActive: boolean;
	readonly cmdLine: string;
	readonly hasSelectionMenu: boolean;
}): "abort" | "dismiss" | "none" {
	if (state.interactionActive) return "abort";
	if (state.cmdLine.trim() || state.hasSelectionMenu) return "dismiss";
	return "none";
}

function replSelectionEvent(selection: SelectionTarget, point?: Vec3): InteractionEvent {
	return point ? { kind: "selection.changed", targets: [selection], point, modifiers: {} } : { kind: "selection.changed", targets: [selection], modifiers: {} };
}

function replSelectionAccepted(accept: readonly TopologyEntityKind[], selection: SelectionTarget | null): selection is SelectionTarget {
	return Boolean(selection && accept.includes(selection.kind));
}

/** @emoji 🪩 Memoized `DocumentHistory` for REPL hosts. */
export function useDocumentHistory(): DocumentHistory {
	return useMemo(() => new DocumentHistory(), []);
}

/** @emoji 🪩 Labels + capability mirror for undo/redo chrome (uses `InteractionSnapshot.capabilities`). */
export function getReplHistoryPresentation(
	spec: InteractionSpec,
	snap: InteractionSnapshot,
	history: DocumentHistory,
): { readonly canUndo: boolean; readonly canRedo: boolean; readonly undoLabel: string; readonly redoLabel: string } {
	const active = isInteractionSessionActive(spec, snap.state);
	const u = history.peekUndo()?.label ?? "";
	const r = history.peekRedo()?.label ?? "";
	return {
		canUndo: snap.capabilities.canUndo,
		canRedo: snap.capabilities.canRedo,
		undoLabel: active ? "Interaction input" : u,
		redoLabel: active ? "Interaction input" : r,
	};
}

/** @emoji 🪩 Subscribes to runtime revisions and derives REPL undo/redo labels. */
export function useReplHistoryState(rt: InteractionRuntime, spec: InteractionSpec, history: DocumentHistory) {
	const snap = useInteractionSnapshot(rt);
	return useMemo(() => getReplHistoryPresentation(spec, snap, history), [spec, snap, history]);
}

export interface InteractionReplProps {
	readonly interactions: readonly SpatialInteraction[];
	readonly interactionId: string;
	readonly spec: InteractionSpec;
	readonly onInteractionId: (id: string) => void;
	readonly runtime: InteractionRuntime;
	readonly history: DocumentHistory;
	readonly document: ModelDocument;
	readonly geometry: SpatialPickGeometry | null;
	readonly derived?: DerivedViewService | null;
	readonly asideExtra?: ReactNode;
	readonly archivedBoxLayouts?: readonly ArchivedBoxLayout[];
	/** @emoji 🔁 When host bumps this positive counter for the same interaction, `cancel()` then `start` without remounting GL. */
	readonly sessionRestartNonce?: number;
}

/** @emoji 🪩 Full spatial REPL: canvas, interaction palette, history controls, last response. */
export function InteractionRepl({
	interactions,
	interactionId,
	spec,
	onInteractionId,
	runtime: rt,
	history,
	document: documentModel,
	geometry,
	derived,
	asideExtra,
	archivedBoxLayouts = [],
	sessionRestartNonce = 0,
}: InteractionReplProps): ReactNode {
	const snapshot = useInteractionSnapshot(rt);
	const documentArchivedBoxLayouts = useMemo(() => archivedBoxesFromHistory(history), [history, snapshot.revision]);
	const allArchivedBoxLayouts = useMemo(
		() => [...documentArchivedBoxLayouts, ...archivedBoxLayouts],
		[documentArchivedBoxLayouts, archivedBoxLayouts],
	);
	const baseDisplay = useMemo(() => replBaseDisplayForHistory(snapshot), [snapshot]);
	const mergedDisplay = useMemo(
		() => mergeDisplayWithArchivedBoxes(baseDisplay, allArchivedBoxLayouts),
		[baseDisplay, allArchivedBoxLayouts],
	);
	const [cmdLine, setCmdLine] = useState("");
	const [activeIndex, setActiveIndex] = useState(0);
	const [selectionKindToggles, setSelectionKindToggles] = useState<Record<SpatialPickTargetKind, boolean>>(() =>
		defaultSpatialPickKindToggles(),
	);
	const [pickViewKind, setPickViewKind] = useState<SpatialPickViewKind>("raw");
	const [derivedRevision, setDerivedRevision] = useState(0);
	const [selectionMenu, setSelectionMenu] = useState<SpatialSelectionRequest | null>(null);
	const [hoveredPickKey, setHoveredPickKey] = useState<string | null>(null);
	const [selectedPickKey, setSelectedPickKey] = useState<string | null>(null);
	const [selectedSelectionTarget, setSelectedSelectionTarget] = useState<SelectionTarget | null>(null);
	const cmdRef = useRef<HTMLInputElement>(null);
	const setCmdLineRef = useRef(setCmdLine);
	const suppressAutoStartOnceRef = useRef(false);
	useEffect(() => {
		setCmdLineRef.current = setCmdLine;
	}, [setCmdLine]);

	const dismissReplChrome = useCallback(() => {
		setCmdLine("");
		setSelectionMenu(null);
		setHoveredPickKey(null);
	}, []);

	const cancelActiveInteraction = useCallback(() => {
		if (!abortActiveInteractionSession(rt)) return false;
		suppressAutoStartOnceRef.current = true;
		setSelectedPickKey(null);
		setSelectedSelectionTarget(null);
		dismissReplChrome();
		return true;
	}, [rt, dismissReplChrome]);

	const interactionActive = isInteractionSessionActive(spec, snapshot.state);

	const handleEscapeKey = useCallback(() => {
		switch (replEscapeAction({ interactionActive, cmdLine, hasSelectionMenu: selectionMenu !== null })) {
			case "abort":
				cancelActiveInteraction();
				return;
			case "dismiss":
				dismissReplChrome();
				return;
			default:
				return;
		}
	}, [interactionActive, cmdLine, selectionMenu, dismissReplChrome, cancelActiveInteraction]);

	const startRuntime = useCallback(async () => {
		const accept = rt.listActiveSelectionAccept() as readonly TopologyEntityKind[];
		if (replSelectionAccepted(accept, selectedSelectionTarget)) await rt.send(replSelectionEvent(selectedSelectionTarget));
	}, [rt, selectedSelectionTarget]);

	useEffect(() => {
		if (suppressAutoStartOnceRef.current) {
			suppressAutoStartOnceRef.current = false;
			return;
		}
		const accept = rt.listActiveSelectionAccept() as readonly TopologyEntityKind[];
		if (replSelectionAccepted(accept, selectedSelectionTarget)) void rt.send(replSelectionEvent(selectedSelectionTarget));
	}, [rt, selectedSelectionTarget]);

	useEffect(() => {
		if (sessionRestartNonce <= 0) return;
		rt.cancel();
		void startRuntime();
	}, [sessionRestartNonce, rt, startRuntime]);

	const repeatCurrentInteraction = useCallback(() => {
		rt.cancel();
		void startRuntime();
	}, [rt, startRuntime]);

	useEffect(() => {
		if (!derived) return;
		const topo = documentModel.topology;
		let cancelled = false;
		void derived.refresh(topo).then(() => {
			if (!cancelled) setDerivedRevision((n) => n + 1);
		});
		return () => {
			cancelled = true;
		};
	}, [derived, documentModel.topology, geometry, snapshot.revision]);

	useEffect(() => {
		setSelectionMenu(null);
		setHoveredPickKey(null);
	}, [geometry, snapshot.state, derivedRevision]);

	useEffect(() => {
		setSelectedPickKey(null);
		setSelectedSelectionTarget(null);
	}, [geometry, derivedRevision]);

	const runtimeSelectionAccept = useMemo(() => rt.listActiveSelectionAccept(), [rt, snapshot.state]);
	const activeSelectionAccept = useMemo(
		() => (runtimeSelectionAccept.length > 0 ? runtimeSelectionAccept : interactionActive ? [] : SPATIAL_PICK_TARGET_KINDS),
		[runtimeSelectionAccept, interactionActive],
	);
	const activePickViewKinds = useMemo(() => spatialPickViewKinds(pickViewKind), [pickViewKind]);
	const analyticSummary = useMemo(() => {
		if (!derived || pickViewKind !== "analytic") return null;
		const topo = documentModel.topology;
		return {
			surfaces: derived.computeSurfaces(topo),
			parts: derived.computeParts(topo),
		};
	}, [derived, documentModel.topology, pickViewKind, derivedRevision, snapshot.revision]);

	useEffect(() => {
		if (activeSelectionAccept.length === 0 || activePickViewKinds.some((kind) => activeSelectionAccept.includes(kind))) return;
		const nextViewKind: SpatialPickViewKind = SPATIAL_ANALYTIC_PICK_TARGET_KINDS.some((kind) => activeSelectionAccept.includes(kind))
			? "analytic"
			: "raw";
		if (nextViewKind !== pickViewKind) {
			setPickViewKind(nextViewKind);
			setSelectionMenu(null);
			setHoveredPickKey(null);
		}
	}, [activeSelectionAccept, activePickViewKinds, pickViewKind]);

	const dispatchSelectionTarget = useCallback(
		(target: SpatialPickTarget, modifiers: InteractionEvent["modifiers"] = {}, point?: Vec3) => {
			const selection = spatialSelectionTarget(target);
			setSelectionMenu(null);
			setHoveredPickKey(null);
			setSelectedPickKey(spatialSelectionTargetKey(selection));
			setSelectedSelectionTarget(selection);
			void rt.send({ ...replSelectionEvent(selection, point), modifiers });
		},
		[rt],
	);

	const onSelectionRequest = useCallback(
		(request: SpatialSelectionRequest) => {
			if (request.targets.length === 1) {
				dispatchSelectionTarget(request.targets[0]!, request.modifiers, request.point);
				return;
			}
			setSelectionMenu(request);
			setHoveredPickKey(request.targets[0] ? spatialPickTargetKey(request.targets[0]) : null);
		},
		[dispatchSelectionTarget],
	);

	const onHoverTarget = useCallback((target: SpatialPickTarget | null) => {
		setHoveredPickKey(target ? spatialPickTargetKey(target) : null);
	}, []);

	const onSpatialInteractionEvent = useCallback(
		(ev: InteractionEvent) => {
			if (ev.kind === "pointer.down") {
				const st = rt.getSnapshot().state;
				const hi = rt.getSnapshot().spatialInteraction.heightConfirmState;
				const snapEv = (ev as { snap?: { kind: string; id: string } }).snap;
				if (hi && st === hi && !snapEv) {
					void rt.send({ kind: "confirm", modifiers: (ev as { modifiers?: Record<string, unknown> }).modifiers ?? {} });
					return;
				}
				if (snapEv && activeSelectionAccept.length > 0 && activeSelectionAccept.includes(snapEv.kind as TopologyEntityKind)) {
					const kind = snapEv.kind as TopologyEntityKind;
					const selection: SelectionTarget =
						kind === "surface" || kind === "part"
							? { kind, id: snapEv.id, editable: false }
							: { kind, id: snapEv.id, editable: true };
					setSelectedPickKey(spatialSelectionTargetKey(selection));
					setSelectedSelectionTarget(selection);
					void rt.send({ ...replSelectionEvent(selection, (ev as { point?: Vec3 }).point), modifiers: (ev as { modifiers?: Record<string, unknown> }).modifiers ?? {} });
					return;
				}
			}
			if (ev.kind === "pointer.down" || ev.kind === "pointer.move" || ev.kind === "contextmenu") void rt.send(ev);
		},
		[rt, activeSelectionAccept],
	);

	const dispatchTransition = useCallback(
		(row: InteractionKeybindRow) => {
			const ev = replBuildDispatchEvent(row, { interactionId: spec.id, topo: documentModel.topology });
			if (ev) void rt.send(ev);
		},
		[rt, spec.id, documentModel.topology],
	);

	const transitionRows = useMemo(() => listKeyedInteractionTransitions(spec, snapshot.state), [spec, snapshot.state]);

	const allSuggestions = useMemo((): ReplSuggestion[] => {
		const out: ReplSuggestion[] = [];
		for (const p of interactions) {
			out.push({
				kind: "interaction",
				key: p.key,
				label: p.label,
				detail: p.id,
				interactionId: p.id,
				onRun: () => onInteractionId(p.id),
			});
		}
		for (const row of transitionRows) {
			out.push({
				kind: "transition",
				key: row.key,
				label: row.label,
				detail: row.eventKind,
				transition: row,
				onRun: () => dispatchTransition(row),
			});
		}
		return out;
	}, [interactions, transitionRows, onInteractionId, dispatchTransition]);

	const filtered = useMemo(() => replPaletteRows(cmdLine, allSuggestions), [cmdLine, allSuggestions]);
	const completionSuffix = useMemo(
		() => replActiveCompletionSuffix(cmdLine, filtered, activeIndex),
		[cmdLine, filtered, activeIndex],
	);

	useEffect(() => {
		setActiveIndex((i) => (filtered.length ? Math.min(i, filtered.length - 1) : 0));
	}, [filtered.length, cmdLine]);

	const runSuggestion = useCallback((s: ReplSuggestion) => {
		s.onRun();
		setCmdLine("");
		setActiveIndex(0);
	}, []);

	const trySubmitLine = useCallback((): boolean => {
		const raw = cmdLine.trim();
		if (!raw) return false;
		const valEv = replTryParseValueInteraction(raw, spec, rt.getSnapshot().state);
		if (valEv) {
			void rt.send(valEv);
			setCmdLine("");
			return true;
		}
		const interactionHit = resolveSpatialInteractionKey(raw);
		if (interactionHit) {
			onInteractionId(interactionHit.id);
			setCmdLine("");
			return true;
		}
		const rows = listKeyedInteractionTransitions(spec, rt.getSnapshot().state);
		for (const row of rows) {
			if (row.eventKind === "set.height" || row.eventKind === "set.distance" || row.eventKind === "set.footprint") continue;
			if (row.key === raw || row.key.toLowerCase() === raw.toLowerCase() || row.eventKind.toLowerCase() === raw.toLowerCase()) {
				dispatchTransition(row);
				setCmdLine("");
				return true;
			}
		}
		return false;
	}, [cmdLine, spec, rt, dispatchTransition, onInteractionId]);

	const runTransitionRow = useCallback(
		(row: InteractionKeybindRow) => {
			if (row.eventKind.startsWith("set.")) {
				setCmdLine(`${row.key} `);
				window.setTimeout(() => cmdRef.current?.focus(), 0);
				return;
			}
			dispatchTransition(row);
		},
		[dispatchTransition],
	);

	const onInputKeyDown = useCallback(
		(e: KeyboardEvent<HTMLInputElement>) => {
			if (e.key === "Escape") {
				e.preventDefault();
				handleEscapeKey();
				return;
			}
			if (e.key === "ArrowDown" && filtered.length) {
				e.preventDefault();
				setActiveIndex((i) => (i + 1) % filtered.length);
				return;
			}
			if (e.key === "ArrowUp" && filtered.length) {
				e.preventDefault();
				setActiveIndex((i) => (i - 1 + filtered.length) % filtered.length);
				return;
			}
			if (e.key === "Tab" && filtered.length) {
				e.preventDefault();
				const suffix = replActiveCompletionSuffix(cmdLine, filtered, activeIndex);
				if (suffix) {
					setCmdLine(cmdLine + suffix);
					return;
				}
				runSuggestion(filtered[activeIndex] ?? filtered[0]!);
				return;
			}
			if (e.key === "Enter") {
				e.preventDefault();
				if (trySubmitLine()) return;
				if (filtered.length) runSuggestion(filtered[activeIndex]!);
				return;
			}
		},
		[filtered, activeIndex, runSuggestion, trySubmitLine, handleEscapeKey],
	);

	useEffect(() => {
		const onWinCapture = (e: KeyboardEvent) => {
			if (e.defaultPrevented || e.isComposing) return;
			const t = e.target;
			const one = e.key.length === 1 ? e.key : "";
			if (replIsQueryTypingTarget(t)) return;
			if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "z") {
				e.preventDefault();
				e.stopPropagation();
				if (e.shiftKey) rt.redo();
				else rt.undo();
				return;
			}
			if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "y") {
				e.preventDefault();
				e.stopPropagation();
				rt.redo();
				return;
			}
			if (replShouldRepeatInteractionOnSpace(e, { interactionActive, cmdTarget: cmdRef.current })) {
				e.preventDefault();
				e.stopPropagation();
				repeatCurrentInteraction();
				return;
			}
			if (t !== cmdRef.current && e.key === "Backspace") {
				e.preventDefault();
				e.stopPropagation();
				cmdRef.current?.focus();
				setCmdLineRef.current((prev) => prev.slice(0, -1));
				return;
			}
			if (t !== cmdRef.current && e.key === "Escape") {
				e.preventDefault();
				e.stopPropagation();
				cmdRef.current?.focus();
				handleEscapeKey();
				return;
			}
			if (t !== cmdRef.current && e.key === "Enter") {
				e.preventDefault();
				e.stopPropagation();
				cmdRef.current?.focus();
				if (cmdLine.trim()) void trySubmitLine();
				return;
			}
			if (!one || e.ctrlKey || e.metaKey || e.altKey) return;
			if (t === cmdRef.current) return;
			e.preventDefault();
			e.stopPropagation();
			cmdRef.current?.focus();
			setCmdLineRef.current((prev) => `${prev}${one}`);
		};
		window.addEventListener("keydown", onWinCapture, true);
		return () => window.removeEventListener("keydown", onWinCapture, true);
	}, [rt, cmdLine, trySubmitLine, handleEscapeKey, interactionActive, repeatCurrentInteraction]);

	const onScenePointerMove = useCallback(
		(p: Vec3) => {
			void rt.send({ kind: "pointer.move", point: p, modifiers: {} });
		},
		[rt],
	);

	const pointerMoveActive = useMemo(() => {
		const si = snapshot.spatialInteraction;
		return (
			si.spatialGroundPick &&
			(si.groundPointerMoveStates.includes(snapshot.state) ||
				si.heightDragStates.includes(snapshot.state) ||
				si.verticalRodStates.includes(snapshot.state))
		);
	}, [snapshot.state, snapshot.spatialInteraction]);

	const pickPlaneOn = snapshot.spatialInteraction.spatialGroundPick
		? !snapshot.spatialInteraction.pickDisabledStates.includes(snapshot.state)
		: false;

	const lr = snapshot.lastResponse;

	return (
		<div
			style={{
				display: "flex",
				height: "100vh",
				fontFamily: "system-ui",
				color: "#e8e8f0",
				background: "#080810",
			}}
		>
			<div style={{ flex: 1, minWidth: 0 }} key={interactionId}>
				<InteractionCanvas>
					<InteractionSpatialView
						snapshot={snapshot}
						onInteractionEvent={onSpatialInteractionEvent}
						onScenePointerMove={pointerMoveActive ? onScenePointerMove : undefined}
						pickEnabled={pickPlaneOn}
						geometry={geometry}
						pickViewKind={pickViewKind}
						derived={derived}
						derivedRevision={derivedRevision}
						displayModel={mergedDisplay}
						selectionAccept={activeSelectionAccept}
						selectionKindToggles={selectionKindToggles}
						hoverKindToggles={selectionKindToggles}
						hoveredTargetKey={hoveredPickKey}
						selectedTargetKey={selectedPickKey}
						onSelectionRequest={onSelectionRequest}
						onHoverTarget={onHoverTarget}
					/>
				</InteractionCanvas>
				{selectionMenu ? (
					<div
						onPointerDown={(e) => e.stopPropagation()}
						style={{
							position: "fixed",
							left: Math.min(selectionMenu.client.x + 8, window.innerWidth - 230),
							top: Math.min(selectionMenu.client.y + 8, window.innerHeight - 220),
							width: 220,
							maxHeight: 210,
							overflowY: "auto",
							background: "#10101a",
							border: "1px solid #4c5a78",
							borderRadius: 7,
							boxShadow: "0 10px 28px rgba(0,0,0,0.55)",
							zIndex: 10080,
							padding: 4,
						}}
					>
						<div style={{ fontSize: 11, opacity: 0.7, padding: "4px 6px" }}>Select target</div>
						{selectionMenu.targets.map((target) => {
							const key = spatialPickTargetKey(target);
							const active = hoveredPickKey === key;
							return (
								<button
									key={key}
									type="button"
									onPointerEnter={() => setHoveredPickKey(selectionKindToggles[target.kind] ? key : null)}
									onPointerLeave={() => setHoveredPickKey(null)}
									onPointerDown={(e) => {
										e.preventDefault();
										e.stopPropagation();
										dispatchSelectionTarget(target, selectionMenu.modifiers, selectionMenu.point);
									}}
									style={{
										display: "block",
										width: "100%",
										border: "none",
										borderRadius: 5,
										padding: "6px 7px",
										textAlign: "left",
										background: active ? "#233b5d" : "transparent",
										color: "#e8e8f0",
										cursor: "pointer",
										fontSize: 12,
									}}
								>
									<span style={{ opacity: 0.7 }}>{target.kind}</span>{" "}
									<code style={{ color: "#ffffff" }}>{target.id}</code>
								</button>
							);
						})}
					</div>
				) : null}
			</div>
			<aside
				style={{
					width: 360,
					padding: 12,
					background: "#12121c",
					borderLeft: "1px solid #2a2a3a",
					display: "flex",
					flexDirection: "column",
					gap: 10,
					overflow: "auto",
					position: "relative",
					zIndex: 2,
				}}
			>
				<strong>Spatial play</strong>
				<div style={{ display: "flex", flexWrap: "wrap", gap: 6 }}>
					{transitionRows.map((row) => (
						<button
							key={`${row.key}-${row.eventKind}-${row.label}`}
							type="button"
							onClick={() => runTransitionRow(row)}
							style={{
								padding: "5px 7px",
								borderRadius: 6,
								border: "1px solid #2e3a52",
								background: "#182238",
								color: "#e8e8f0",
								cursor: "pointer",
								fontSize: 12,
							}}
						>
							<span style={{ textDecoration: "underline", fontWeight: 700 }}>{row.key}</span> {row.label}
						</button>
					))}
				</div>
				<div
					style={{
						display: "grid",
						borderRadius: 6,
						background: "#0e0e16",
						border: "1px solid #3a4762",
					}}
				>
					<input
						ref={cmdRef}
						type="text"
						autoComplete="off"
						spellCheck={false}
						value={cmdLine}
						onChange={(e) => setCmdLine(e.target.value)}
						onKeyDown={onInputKeyDown}
						placeholder="Type an interaction or transition"
						style={{
							gridArea: "1 / 1",
							width: "100%",
							boxSizing: "border-box",
							padding: "8px 9px",
							borderRadius: 6,
							background: "transparent",
							color: "#e8e8f0",
							border: "none",
							outline: "none",
							fontSize: 13,
							fontFamily: "inherit",
							lineHeight: "normal",
						}}
					/>
					{completionSuffix ? (
						<div
							aria-hidden
							style={{
								gridArea: "1 / 1",
								pointerEvents: "none",
								boxSizing: "border-box",
								padding: "8px 9px",
								fontSize: 13,
								fontFamily: "inherit",
								lineHeight: "normal",
								whiteSpace: "pre",
								overflow: "hidden",
								color: "#e8e8f0",
							}}
						>
							<span style={{ color: "transparent" }}>{cmdLine}</span>
							<span style={{ opacity: 0.45 }}>{completionSuffix}</span>
						</div>
					) : null}
				</div>
				{asideExtra}
				<div style={{ display: "flex", flexDirection: "column", gap: 6, fontSize: 12 }}>
					<span>Pick view</span>
					<div style={{ display: "flex", gap: 6 }}>
						{(["raw", "analytic"] as const).map((viewKind) => {
							const active = pickViewKind === viewKind;
							return (
								<button
									key={viewKind}
									type="button"
									onClick={() => {
										setPickViewKind(viewKind);
										setSelectionMenu(null);
										setHoveredPickKey(null);
										setSelectedPickKey(null);
									}}
									style={{
										padding: "5px 8px",
										borderRadius: 999,
										border: active ? "1px solid #77aaff" : "1px solid #2a2a3a",
										background: active ? "#1f3656" : "#12121c",
										color: "#e8e8f0",
										cursor: "pointer",
										fontSize: 12,
										textTransform: "capitalize",
									}}
								>
									{viewKind}
								</button>
							);
						})}
					</div>
					{analyticSummary ? (
						<div style={{ display: "flex", flexDirection: "column", gap: 4, maxHeight: 160, overflowY: "auto" }}>
							<span style={{ opacity: 0.75 }}>
								{analyticSummary.surfaces.length} surfaces · {analyticSummary.parts.length} parts
							</span>
							{analyticSummary.parts.map((part) => (
								<span key={String(part.id)} style={{ fontSize: 11, opacity: 0.85 }}>
									{part.overlap} · {String(part.id)}
								</span>
							))}
						</div>
					) : null}
					<span>Selection kinds</span>
					<div style={{ display: "flex", flexWrap: "wrap", gap: 6 }}>
						{activePickViewKinds.map((kind) => {
							const accepted = activeSelectionAccept.length === 0 || activeSelectionAccept.includes(kind);
							return (
								<label
									key={kind}
									style={{
										display: "flex",
										alignItems: "center",
										gap: 4,
										padding: "3px 6px",
										border: "1px solid #2a2a3a",
										borderRadius: 999,
										opacity: accepted ? 1 : 0.45,
										background: selectionKindToggles[kind] ? "#1a2638" : "#12121c",
									}}
								>
									<input
										type="checkbox"
										checked={selectionKindToggles[kind]}
										onChange={(e) => {
											const checked = e.target.checked;
											setSelectionKindToggles((prev) => ({ ...prev, [kind]: checked }));
											setSelectionMenu(null);
											setHoveredPickKey(null);
											if (!checked) setSelectedPickKey((key) => (key?.startsWith(`${kind}:`) ? null : key));
										}}
									/>
									{kind}
								</label>
							);
						})}
					</div>
				</div>
				<div style={{ fontSize: 12, opacity: 0.85 }}>
					{interactionId ? (
						<>
							Interaction <code>{interactionId}</code> · state <code>{snapshot.state}</code> · rev {snapshot.revision}
						</>
					) : (
						<>
							No interaction selected · state <code>{snapshot.state}</code> · rev {snapshot.revision}
						</>
					)}
				</div>
				<div style={{ fontSize: 12, borderTop: "1px solid #2a2a3a", paddingTop: 8 }}>
					<strong>Last response</strong>
					<pre style={{ fontSize: 10, overflow: "auto", maxHeight: 120, margin: "6px 0 0" }}>
						{lr ? JSON.stringify(lr, null, 2) : "—"}
					</pre>
					{snapshot.diagnostics.length ? (
						<ul style={{ fontSize: 11, margin: 0, paddingLeft: 16 }}>
							{snapshot.diagnostics.map((d, i) => (
								<li key={`${d.code}-${i}`}>
									[{d.severity}] {d.code}: {d.message}
								</li>
							))}
						</ul>
					) : null}
				</div>
			</aside>
		</div>
	);
}
// #endregion 🪩Repl

// #region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("@spatial/js-renderer-r3f preview transforms", () => {
		it("bboxWireSegments returns twelve edges", () => {
			const segs = bboxWireSegments([0, 0, 0], [1, 1, 1]);
			expect(segs).toHaveLength(12);
		});

		it("topologyEntityWireSegments uses face boundary edges not bbox diagonals", () => {
			const topo = new TopologyGraph();
			applyTopologyDiff(topo, boxTopologyDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, cellRef("box")));
			const faceId = Object.keys(topo.faces)[0]!;
			const buckets = topologyGeometryBuckets(topo);
			const segs = topologyEntityWireSegments(buckets, "face", faceId);
			expect(segs.length).toBeGreaterThanOrEqual(4);
			expect(segs.length).toBeLessThan(12);
		});

		it("collectTopologyEdgeSegments returns one segment per topology edge", () => {
			const topo = new TopologyGraph();
			applyTopologyDiff(topo, boxTopologyDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, cellRef("box")));
			const segs = collectTopologyEdgeSegments(topologyGeometryBuckets(topo));
			expect(segs.length).toBe(Object.keys(topo.edges).length);
			expect(segs.length).toBe(12);
		});

		it("move-preview translates points by cursor minus from", () => {
			const map = transformPointsForPreviewKind("move-preview", {
				from: [0, 0, 0],
				cursor: [2, 1, 0],
			});
			expect(map([1, 2, 3])).toEqual([3, 3, 3]);
		});

		it("selected-objects preview keeps points unchanged", () => {
			const map = transformPointsForPreviewKind("selected-objects", { cursor: [5, 5, 0] });
			expect(map([1, 0, 0])).toEqual([1, 0, 0]);
		});
	});

	describe("@spatial/js-renderer-r3f layout", () => {
		it("computeBoxPreviewLayout matches footprint and height", () => {
			const L = computeBoxPreviewLayout([0, 0, 0], [2, 3, 0], 4);
			expect(L.scale[0]).toBeCloseTo(2);
			expect(L.scale[1]).toBeCloseTo(3);
			expect(L.scale[2]).toBeCloseTo(4);
			expect(L.position[0]).toBeCloseTo(1);
			expect(L.position[1]).toBeCloseTo(1.5);
			expect(L.position[2]).toBeCloseTo(2);
		});
	});

	describe("@spatial/js-renderer-r3f interaction adapter", () => {
		it("maps pointer event data into interaction events", () => {
			const adapter = createR3FInteractionAdapter();
			const event = {
				point: { x: 1, y: 2, z: 3 },
				altKey: false,
				ctrlKey: true,
				metaKey: false,
				shiftKey: true,
			} as ThreeEvent<PointerEvent>;
			expect(adapter.pointerDown(event)).toEqual({
				kind: "pointer.down",
				point: [1, 2, 3],
				modifiers: { alt: false, ctrl: true, meta: false, shift: true },
			});
		});

		it("creates snap and selection metadata for topology targets", () => {
			const targets = createSpatialPickTargets({
				schema: "spatial.topology/v1",
				revision: 1,
				anchors: [],
				vertices: [{ id: "v0", position: [1, 2, 3] }],
				edges: [],
				wires: [],
				faces: [],
				shells: [],
				cells: [],
				cellComplexes: [],
				clusters: [],
			});
			expect(targets).toEqual([{ kind: "vertex", id: "v0", point: [1, 2, 3] }]);
			expect(createSpatialPickEvent("pointer.down", [9, 9, 9], targets[0]!, { shift: true })).toEqual({
				kind: "pointer.down",
				point: [9, 9, 9],
				modifiers: { shift: true },
				snap: { kind: "vertex", id: "v0", point: [1, 2, 3] },
				selection: { kind: "vertex", id: "v0" },
			});
		});

		it("creates selectable targets for every editable topology kind", () => {
			const targets = createSpatialPickTargets({
				schema: "spatial.topology/v1",
				revision: 1,
				anchors: [{ id: "a0", position: [0.25, 0.25, 0], attachment: { kind: "vertex", id: "v0" } }],
				vertices: [
					{ id: "v0", position: [0, 0, 0] },
					{ id: "v1", position: [1, 0, 0] },
					{ id: "v2", position: [1, 1, 0] },
				],
				edges: [
					{ id: "e0", vertexIds: ["v0", "v1"] },
					{ id: "e1", vertexIds: ["v1", "v2"] },
				],
				wires: [{ id: "w0", edgeIds: ["e0", "e1"] }],
				faces: [{ id: "f0", wireIds: ["w0"] }],
				shells: [{ id: "sh0", faceIds: ["f0"] }],
				cells: [{ id: "c0", shellIds: ["sh0"] }],
				cellComplexes: [{ id: "cc0", cellIds: ["c0"] }],
				clusters: [{ id: "cl0", memberIds: ["c0"] }],
			});
			expect(targets.map((target) => target.kind)).toEqual([
				"anchor",
				"vertex",
				"vertex",
				"vertex",
				"edge",
				"edge",
				"wire",
				"face",
				"shell",
				"cell",
				"cellComplex",
				"cluster",
			]);
			expect(targets.find((target) => target.kind === "cell")?.point).toEqual([2 / 3, 1 / 3, 0]);
		});

		it("creates analytic surface and part targets from derived views", async () => {
			const topo = new TopologyGraph();
			applyTopologyDiff(topo, boxTopologyDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, cellRef("c0")));
			const derived = new DerivedViewService();
			await derived.refresh(topo);
			const targets = createSpatialPickTargets(topo, derived);
			expect(targets.some((t) => t.kind === "surface")).toBe(true);
			expect(targets.some((t) => t.kind === "part")).toBe(true);
			const analytic = filterSpatialPickTargetsByView(targets, "analytic");
			expect(analytic.every((t) => t.kind === "surface" || t.kind === "part")).toBe(true);
			expect(analytic.length).toBeGreaterThan(0);
		});

		it("partitions raw and analytic pick targets", () => {
			const targets: SpatialPickTarget[] = [
				{ kind: "vertex", id: "v0", point: [0, 0, 0] },
				{ kind: "face", id: "f0", point: [0.5, 0.5, 0] },
				{ kind: "surface", id: "surface-f0", point: [0.5, 0.5, 0] },
				{ kind: "part", id: "part-c0", point: [0.5, 0.5, 0.5] },
			];
			expect(filterSpatialPickTargetsByView(targets, "raw").map(spatialPickTargetKey)).toEqual(["vertex:v0", "face:f0"]);
			expect(filterSpatialPickTargetsByView(targets, "analytic").map(spatialPickTargetKey)).toEqual([
				"surface:surface-f0",
				"part:part-c0",
			]);
		});

		it("creates selectable targets for every committed box topology kind", async () => {
			const topo = new TopologyGraph();
			applyTopologyDiff(topo, boxTopologyDiff({ cornerA: [0, 0, 0], cornerB: [2, 3, 0], height: 4 }, cellRef("box-cell")));
			const derived = new DerivedViewService();
			await derived.refresh(topo);
			const targets = createSpatialPickTargets(topo, derived);
			const counts = targets.reduce<Record<string, number>>((acc, target) => {
				acc[target.kind] = (acc[target.kind] ?? 0) + 1;
				return acc;
			}, {});
			expect(counts.vertex).toBe(8);
			expect(counts.edge).toBe(12);
			expect(counts.wire).toBe(6);
			expect(counts.face).toBe(6);
			expect(counts.surface).toBeGreaterThan(0);
			expect(counts.shell).toBe(1);
			expect(counts.cell).toBe(1);
			expect(counts.part).toBeGreaterThan(0);
		});

		it("filters selectable targets by active accept list and kind toggles", () => {
			const targets: SpatialPickTarget[] = [
				{ kind: "vertex", id: "v0", point: [0, 0, 0] },
				{ kind: "edge", id: "e0", point: [0.5, 0, 0] },
				{ kind: "face", id: "f0", point: [0.5, 0.5, 0] },
			];
			expect(filterSpatialPickTargets(targets, ["vertex", "edge"], { edge: false }).map(spatialPickTargetKey)).toEqual([
				"vertex:v0",
			]);
		});

		it("keeps targets rendered when selection or hover toggles enable their kind", () => {
			const targets: SpatialPickTarget[] = [
				{ kind: "vertex", id: "v0", point: [0, 0, 0] },
				{ kind: "edge", id: "e0", point: [0.5, 0, 0] },
				{ kind: "face", id: "f0", point: [0.5, 0.5, 0] },
			];
			const visible = filterSpatialPickTargetsForAnyToggle(
				targets,
				{ vertex: true, edge: false, face: false },
				{ vertex: false, edge: true, face: false },
			);
			expect(visible.map(spatialPickTargetKey)).toEqual(["vertex:v0", "edge:e0"]);
		});

		it("ray-picks overlapping face and surface candidates", () => {
			const targets: SpatialPickTarget[] = [
				{ kind: "face", id: "f0", point: [0.5, 0.5, 0], points: [[0, 0, 0], [1, 0, 0], [1, 1, 0]] },
				{ kind: "surface", id: "surface-f0", point: [0.5, 0.5, 0], points: [[0, 0, 0], [1, 0, 0], [1, 1, 0]] },
			];
			const ray = new THREE.Ray(new THREE.Vector3(0.5, 0.5, 2), new THREE.Vector3(0, 0, -1));
			expect(spatialPickTargetsFromRay(ray, targets, ["face", "surface"], {}).map(spatialPickTargetKey)).toEqual([
				"face:f0",
				"surface:surface-f0",
			]);
		});

		it("carries host selection into interaction selection events", () => {
			const selection: SelectionTarget = { kind: "wire", id: "w0", editable: true };
			expect(replSelectionAccepted(["wire"], selection)).toBe(true);
			expect(replSelectionAccepted(["face"], selection)).toBe(false);
			expect(replSelectionEvent(selection)).toEqual({ kind: "selection.changed", targets: [selection], modifiers: {} });
		});
	});

	describe("@spatial/js-renderer-r3f runtime", () => {
		it("exposes an initial snapshot for the box interaction with a stub kernel", () => {
			class StubKernel implements KernelAdapter {
				readonly id = "stub";
				readonly operations = ["cell.createBox", "entity.tessellate"] as const;
				async createBoxFromCorners() {
					return cellRef("stub");
				}
				async volume() {
					return 0;
				}
				async tessellate() {
					return { positions: new Float32Array(), indices: new Uint32Array() };
				}
			}
			const spec = buildBoxInteractionSpec();
			const runtime = createInteractionRuntime(spec, {
				kernel: new StubKernel(),
				document: { topology: new TopologyGraph(), nodes: [] },
			});
			const snapshot = runtime.getSnapshot();
			expect(snapshot.interactionId).toBe(spec.id);
			expect(snapshot.state).toBe(spec.machine.initial);
		});
	});

	describe("@spatial/js-renderer-r3f repl history", () => {
		it("getReplHistoryPresentation exposes canRedo after document undo", () => {
			class StubKernel implements KernelAdapter {
				readonly id = "stub-repl";
				readonly operations = ["cell.createBox", "entity.tessellate"] as const;
				async createBoxFromCorners() {
					return cellRef("c");
				}
				async volume() {
					return 0;
				}
				async tessellate() {
					return { positions: new Float32Array(), indices: new Uint32Array() };
				}
			}
			const g = new TopologyGraph();
			const mesh = { positions: new Float32Array([0, 0, 0, 1, 0, 0, 0, 1, 0]), indices: new Uint32Array([0, 1, 2]) };
			const d0 = meshFaceTopologyDiff(mesh, "x");
			const inv = applyTopologyDiff(g, d0);
			const hist = new DocumentHistory();
			hist.record({
				id: "m0",
				interactionId: "c",
				label: "L",
				result: { ok: true, errors: [], warnings: [], infos: [], diff: d0, data: null, archiveContext: null },
				backwardsDiff: inv,
			});
			const spec = buildBoxInteractionSpec();
			const rt = createInteractionRuntime(spec, { kernel: new StubKernel(), document: { topology: g, nodes: [] }, history: hist });
			let snap = rt.getSnapshot();
			let pres = getReplHistoryPresentation(spec, snap, hist);
			expect(pres.canUndo).toBe(true);
			expect(pres.canRedo).toBe(false);
			rt.undo();
			snap = rt.getSnapshot();
			pres = getReplHistoryPresentation(spec, snap, hist);
			expect(pres.canRedo).toBe(true);
		});
	});

	describe("@spatial/js-renderer-r3f interaction repl", () => {
		it("exports InteractionRepl and useInteractionRuntime for hosts", () => {
			expect(typeof InteractionRepl).toBe("function");
			expect(typeof useInteractionRuntime).toBe("function");
		});

		it("repeats the current interaction on bare space only while inactive", () => {
			const cmd = document.createElement("input");
			const other = document.createElement("div");
			const notes = document.createElement("textarea");
			expect(
				replShouldRepeatInteractionOnSpace(
					{
						key: " ",
						ctrlKey: false,
						metaKey: false,
						altKey: false,
						defaultPrevented: false,
						isComposing: false,
						target: other,
					},
					{ interactionActive: false, cmdTarget: cmd },
				),
			).toBe(true);
			expect(
				replShouldRepeatInteractionOnSpace(
					{
						key: " ",
						ctrlKey: false,
						metaKey: false,
						altKey: false,
						defaultPrevented: false,
						isComposing: false,
						target: other,
					},
					{ interactionActive: true, cmdTarget: cmd },
				),
			).toBe(false);
			expect(
				replShouldRepeatInteractionOnSpace(
					{
						key: " ",
						ctrlKey: false,
						metaKey: false,
						altKey: false,
						defaultPrevented: false,
						isComposing: false,
						target: cmd,
					},
					{ interactionActive: false, cmdTarget: cmd },
				),
			).toBe(false);
			expect(
				replShouldRepeatInteractionOnSpace(
					{
						key: " ",
						ctrlKey: false,
						metaKey: false,
						altKey: false,
						defaultPrevented: false,
						isComposing: false,
						target: notes,
					},
					{ interactionActive: false, cmdTarget: cmd },
				),
			).toBe(false);
			expect(
				replShouldRepeatInteractionOnSpace(
					{
						key: " ",
						ctrlKey: true,
						metaKey: false,
						altKey: false,
						defaultPrevented: false,
						isComposing: false,
						target: other,
					},
					{ interactionActive: false, cmdTarget: cmd },
				),
			).toBe(false);
		});

		it("escape aborts active interactions before dismissing chrome", () => {
			expect(replEscapeAction({ interactionActive: true, cmdLine: "height 4", hasSelectionMenu: true })).toBe("abort");
			expect(replEscapeAction({ interactionActive: false, cmdLine: "height 4", hasSelectionMenu: false })).toBe("dismiss");
			expect(replEscapeAction({ interactionActive: false, cmdLine: "", hasSelectionMenu: true })).toBe("dismiss");
			expect(replEscapeAction({ interactionActive: false, cmdLine: "", hasSelectionMenu: false })).toBe("none");
		});

		it("autocomplete helpers rank prefix matches and expose inline suffix", () => {
			const all: ReplSuggestion[] = [
				{ kind: "interaction", key: "m", label: "Move", detail: "transform.move", onRun: () => {} },
				{ kind: "interaction", key: "b", label: "Box", detail: "primitive.box", onRun: () => {} },
				{ kind: "transition", key: "c", label: "Confirm", detail: "confirm", onRun: () => {} },
			];
			expect(replFilterSuggestions("", all)).toEqual([]);
			expect(replPaletteRows("", all)).toEqual([]);
			expect(replFilterSuggestions("  ", all)).toEqual([]);
			expect(replPaletteRows("b", all).map((s) => s.key)).toEqual(["b"]);
			expect(replPaletteRows("bo", all).map((s) => s.key)).toEqual(["b"]);
			expect(replCompletionSuffix("b", all[1])).toBe("ox");
			expect(replCompletionSuffix("bo", all[1])).toBe("x");
			expect(replActiveCompletionSuffix("b", replPaletteRows("b", all), 0)).toBe("ox");
			expect(replActiveCompletionSuffix("bo", replPaletteRows("bo", all), 0)).toBe("x");
		});

		it("derives persistent box footprints from document history", () => {
			const g = new TopologyGraph();
			const mesh = { positions: new Float32Array([0, 0, 0, 1, 0, 0, 0, 1, 0]), indices: new Uint32Array([0, 1, 2]) };
			const d0 = meshFaceTopologyDiff(mesh, "hist-box");
			const inv = applyTopologyDiff(g, d0);
			const hist = new DocumentHistory();
			hist.record({
				id: "box-1",
				interactionId: "primitive.box",
				label: "Box",
				result: {
					ok: true,
					errors: [],
					warnings: [],
					infos: [],
					diff: d0,
					data: null,
					archiveContext: { origin: [0, 0, 0], corner: [2, 3, 0], height: 4 },
				},
				backwardsDiff: inv,
			});
			expect(archivedBoxesFromHistory(hist)).toEqual([{ cornerA: [0, 0, 0], cornerB: [2, 3, 0], height: 4 }]);
			hist.undo({ topology: g, nodes: [] });
			expect(archivedBoxesFromHistory(hist)).toEqual([]);
		});
	});
}
// #endregion 🧪Tests

export type { MeshPreview };
