// #region 🧲Header
/** @emoji 🎬 `@spatial/js-renderer-r3f` — R3F `InteractionDisplay`, ground picking, interaction adapter, `InteractionCanvas`, and snapshot hooks. See `spatial/fixtures/box.interaction.json`. */
// #endregion 🧲Header

// #region 📥Imports
import { Line, OrbitControls, Text } from "@react-three/drei";
import { Canvas, type ThreeEvent } from "@react-three/fiber";
import { useCallback, useEffect, useMemo, useRef, useState, useSyncExternalStore, type KeyboardEvent, type ReactNode } from "react";
import { MOUSE } from "three";
import * as THREE from "three";
import {
	applyTopologyDiff,
	buildBoxInteractionSpec,
	cellRef,
	createInteractionRuntime,
	DocumentHistory,
	isInteractionSessionActive,
	isEmptyTopologyDiff,
	listKeyedInteractionTransitions,
	meshFaceTopologyDiff,
	resolveSpatialInteractionPresetKey,
	TopologyGraph,
	type InteractionEvent,
	type InteractionKeybindRow,
	type InteractionRuntime,
	type InteractionRuntimeOptions,
	type InteractionSnapshot,
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
	type ShellRecord,
	type MeshPreview,
	type SpatialInteractionPreset,
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

const raycastNone: THREE.Object3D["raycast"] = () => undefined;
// #endregion 📐Layout

// #region 🧲TopologyTargets
export type SpatialPickKind = "pointer.down" | "pointer.move";

export type SpatialPickTargetKind = Extract<
	TopologyEntityKind,
	"vertex" | "edge" | "wire" | "face" | "shell" | "cell" | "cellComplex" | "cluster"
>;

export const SPATIAL_PICK_TARGET_KINDS: readonly SpatialPickTargetKind[] = [
	"vertex",
	"edge",
	"wire",
	"face",
	"shell",
	"cell",
	"cellComplex",
	"cluster",
];

export type SpatialPickKindToggles = Partial<Record<SpatialPickTargetKind, boolean>>;

export interface SpatialPickTarget {
	readonly kind: SpatialPickTargetKind;
	readonly id: string;
	readonly point: Vec3;
	readonly points?: readonly Vec3[];
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

function defaultSpatialPickKindToggles(): Record<SpatialPickTargetKind, boolean> {
	return Object.fromEntries(SPATIAL_PICK_TARGET_KINDS.map((kind) => [kind, true])) as Record<SpatialPickTargetKind, boolean>;
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
	return edge.vertexIds.map((id) => topologyVertexPoint(vertices, id)).filter((p): p is Vec3 => Boolean(p));
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
	return [];
}

/** @emoji 🧲 Builds renderer-side snap/select targets from optional factory topology geometry. */
export function createSpatialPickTargets(geometry: SpatialPickGeometry | null | undefined): readonly SpatialPickTarget[] {
	if (!geometry) return [];
	const buckets = topologyGeometryBuckets(geometry);
	const targets: SpatialPickTarget[] = [];
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
				point: target.point,
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

function DisplayItemNode({ item }: { readonly item: DisplayItem }): ReactNode {
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
		case "entity-highlight":
			return null;
		default:
			return null;
	}
}

/** @emoji 🖼️ Maps `DisplayModel.items` to R3F nodes (must live under `<Canvas>`). */
export function InteractionDisplay({ model }: { readonly model: DisplayModel }): ReactNode {
	return (
		<group>
			{model.items.map((item) => (
				<group key={item.id}>
					<DisplayItemNode item={item} />
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
	readonly onPointerMove?: (point: Vec3) => void;
	readonly pointerMoveEnabled?: boolean;
}

export function GroundPickPlane({
	planeZ = 0,
	enabled = true,
	onPick,
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
	const onPointerMoveH = (e: ThreeEvent<PointerEvent>) => {
		if (!moveOn || !onPointerMove) return;
		e.stopPropagation();
		const p = e.point;
		onPointerMove([p.x, p.y, planeZ] as unknown as Vec3);
	};
	return (
		<mesh position={[0, 0, planeZ]} onPointerDown={onPointerDown} onPointerMove={onPointerMoveH}>
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

function spatialPickTargetsByKey(targets: readonly SpatialPickTarget[]): ReadonlyMap<string, SpatialPickTarget> {
	return new Map(targets.map((target) => [spatialPickTargetKey(target), target]));
}

function spatialPickKeyFromObject(object: THREE.Object3D): string | null {
	let cur: THREE.Object3D | null = object;
	while (cur) {
		const key = cur.userData.spatialPickKey;
		if (typeof key === "string") return key;
		cur = cur.parent;
	}
	return null;
}

function spatialPickTargetsFromIntersections(
	intersections: readonly { readonly object: THREE.Object3D }[],
	targetsByKey: ReadonlyMap<string, SpatialPickTarget>,
	fallback: SpatialPickTarget,
): SpatialPickTarget[] {
	const seen = new Set<string>();
	const out: SpatialPickTarget[] = [];
	for (const intersection of intersections) {
		const key = spatialPickKeyFromObject(intersection.object);
		if (!key || seen.has(key)) continue;
		const target = targetsByKey.get(key);
		if (!target) continue;
		seen.add(key);
		out.push(target);
	}
	return out.length ? out : [fallback];
}

function targetStyle(target: SpatialPickTarget, hovered: boolean, selected: boolean): { color: string; emissive: string; opacity: number; lineWidth: number } {
	if (selected) return { color: "#ff77bb", emissive: "#551233", opacity: target.kind === "vertex" ? 1 : 0.34, lineWidth: 9 };
	if (hovered) return { color: "#66e8ff", emissive: "#003844", opacity: target.kind === "vertex" ? 1 : 0.28, lineWidth: 8 };
	if (target.kind === "vertex") return { color: "#ffdf7a", emissive: "#4a3000", opacity: 1, lineWidth: 5 };
	if (target.kind === "edge" || target.kind === "wire") return { color: "#ffd166", emissive: "#4a3000", opacity: 0.8, lineWidth: 5 };
	return { color: "#f6c85f", emissive: "#332100", opacity: 0.16, lineWidth: 5 };
}

function SpatialPickTargetNode({
	target,
	targetsByKey,
	onInteractionEvent,
	onPick,
	onPointerMove,
	onSelectionRequest,
	onHoverTarget,
	pointerMoveEnabled,
	selectionAccept,
	kindToggles,
	hoveredTargetKey,
	selectedTargetKey,
}: {
	readonly target: SpatialPickTarget;
	readonly targetsByKey: ReadonlyMap<string, SpatialPickTarget>;
	readonly onInteractionEvent?: (event: InteractionEvent) => void;
	readonly onPick?: (point: Vec3, event: InteractionEvent) => void;
	readonly onPointerMove?: (point: Vec3, event: InteractionEvent) => void;
	readonly onSelectionRequest?: (request: SpatialSelectionRequest) => void;
	readonly onHoverTarget?: (target: SpatialPickTarget | null) => void;
	readonly pointerMoveEnabled: boolean;
	readonly selectionAccept: readonly TopologyEntityKind[];
	readonly kindToggles: SpatialPickKindToggles;
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
		const candidates = filterSpatialPickTargets(
			spatialPickTargetsFromIntersections(e.intersections, targetsByKey, target),
			selectionAccept,
			kindToggles,
		);
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
		onHoverTarget?.(target);
		if (!pointerMoveEnabled) return;
		emit("pointer.move", e);
	};
	const onPointerOut = () => onHoverTarget?.(null);
	const userData = { spatialPickKey: targetKey };
	if (target.kind === "vertex") {
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

/** @emoji 🧲 Renders optional factory geometry as pickable snap/select targets. */
export function SpatialPickGeometryLayer({
	geometry,
	onInteractionEvent,
	onPick,
	onPointerMove,
	onSelectionRequest,
	onHoverTarget,
	pointerMoveEnabled = false,
	selectionAccept = [],
	kindToggles = {},
	hoveredTargetKey,
	selectedTargetKey,
}: {
	readonly geometry?: SpatialPickGeometry | null;
	readonly onInteractionEvent?: (event: InteractionEvent) => void;
	readonly onPick?: (point: Vec3, event: InteractionEvent) => void;
	readonly onPointerMove?: (point: Vec3, event: InteractionEvent) => void;
	readonly onSelectionRequest?: (request: SpatialSelectionRequest) => void;
	readonly onHoverTarget?: (target: SpatialPickTarget | null) => void;
	readonly pointerMoveEnabled?: boolean;
	readonly selectionAccept?: readonly TopologyEntityKind[];
	readonly kindToggles?: SpatialPickKindToggles;
	readonly hoveredTargetKey?: string | null;
	readonly selectedTargetKey?: string | null;
}): ReactNode {
	const topoRevision =
		geometry && typeof geometry === "object" && "revision" in geometry
			? Number((geometry as { revision?: unknown }).revision)
			: 0;
	const targets = useMemo(() => createSpatialPickTargets(geometry), [geometry, topoRevision]);
	const enabledTargets = useMemo(() => filterSpatialPickTargets(targets, [], kindToggles), [targets, kindToggles]);
	const targetsByKey = useMemo(() => spatialPickTargetsByKey(enabledTargets), [enabledTargets]);
	return (
		<group>
			{enabledTargets.map((target) => (
				<SpatialPickTargetNode
					key={`${target.kind}:${target.id}`}
					target={target}
					targetsByKey={targetsByKey}
					onInteractionEvent={onInteractionEvent}
					onPick={onPick}
					onPointerMove={onPointerMove}
					onSelectionRequest={onSelectionRequest}
					onHoverTarget={onHoverTarget}
					pointerMoveEnabled={pointerMoveEnabled}
					selectionAccept={selectionAccept}
					kindToggles={kindToggles}
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

/** @emoji 🪩 Root `<Canvas>` preset for factory viewports. */
export function InteractionCanvas({ children }: InteractionCanvasProps): ReactNode {
	return (
		<Canvas style={{ height: "100%", width: "100%" }} camera={{ position: [10, 10, 8], fov: 45 }}>
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
	/** @emoji 🖼️ When set, drives `InteractionDisplay` instead of `snapshot.display` (e.g. merged archived footprints). */
	readonly displayModel?: DisplayModel;
	readonly selectionAccept?: readonly TopologyEntityKind[];
	readonly pickKindToggles?: SpatialPickKindToggles;
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
	displayModel,
	selectionAccept = [],
	pickKindToggles = {},
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
	const pickPlaneEnabled =
		hostPickGate && si.spatialGroundPick && !si.pickDisabledStates.includes(snapshot.state);
	const onGroundPickEvent = (point: Vec3) => {
		const event = createSpatialPickEvent("pointer.down", point, null);
		onInteractionEvent?.(event);
		onGroundPick?.(point, event);
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
				onPointerMove={onScenePointerMoveEvent}
				pointerMoveEnabled={groundMoveOn}
			/>
			<SpatialPickGeometryLayer
				geometry={geometry}
				onInteractionEvent={onInteractionEvent}
				onPick={undefined}
				onPointerMove={onScenePointerMove}
				onSelectionRequest={onSelectionRequest}
				onHoverTarget={onHoverTarget}
				pointerMoveEnabled={groundMoveOn || heightMoveOn || zRodMoveOn}
				selectionAccept={selectionAccept}
				kindToggles={pickKindToggles}
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
			<InteractionDisplay model={displayModel ?? snapshot.display} />
			{committedMesh ? <TessellatedCommitMesh mesh={committedMesh} /> : null}
		</>
	);
}
// #endregion 🪩Canvas

// #region 🪩Repl
type ReplSuggestKind = "preset" | "transition" | "host";

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
	}
	return null;
}

function replSuggestionHaystack(s: ReplSuggestion): string {
	return `${s.key} ${s.label} ${s.detail}`.toLowerCase();
}

function replFilterSuggestions(query: string, all: readonly ReplSuggestion[]): ReplSuggestion[] {
	const q = query.trim().toLowerCase();
	if (!q) return [...all];
	return all.filter((s) => replSuggestionHaystack(s).includes(q));
}

function replPresetSuggestionsFrom(all: readonly ReplSuggestion[]): ReplSuggestion[] {
	return all.filter((s) => s.kind === "preset");
}

function replPaletteRows(cmdLine: string, all: readonly ReplSuggestion[]): ReplSuggestion[] {
	const fac = replPresetSuggestionsFrom(all);
	const hit = replFilterSuggestions(cmdLine, all);
	if (!cmdLine.trim()) return hit;
	const rest = hit.filter((s) => s.kind !== "preset");
	const seen = new Set<string>();
	const out: ReplSuggestion[] = [];
	for (const s of [...fac, ...rest]) {
		const k = `${s.kind}:${s.key}:${s.detail}`;
		if (seen.has(k)) continue;
		seen.add(k);
		out.push(s);
	}
	return out;
}

function replIsTextTypingTarget(t: EventTarget | null): boolean {
	if (!t || !(t instanceof HTMLElement)) return false;
	if (t.isContentEditable) return true;
	const tag = t.tagName;
	if (tag === "TEXTAREA" || tag === "SELECT") return true;
	if (tag !== "INPUT") return false;
	const ty = (t as HTMLInputElement).type;
	return !["button", "checkbox", "radio", "range", "reset", "submit"].includes(ty);
}

function replPresentationWithUnderlinedKey(key: string, label: string): ReactNode {
	return (
		<>
			<span style={{ textDecoration: "underline", fontWeight: 700 }}>{key}</span>
			{label}
		</>
	);
}

function replPresetFromShortcutKey(evKey: string, presets: readonly SpatialInteractionPreset[]): SpatialInteractionPreset | null {
	if (evKey.length !== 1) return null;
	const k = evKey.toLowerCase();
	for (const p of presets) {
		if (p.key.toLowerCase() === k) return p;
	}
	return null;
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
	readonly presets: readonly SpatialInteractionPreset[];
	readonly interactionId: string;
	readonly spec: InteractionSpec;
	readonly onInteractionId: (id: string) => void;
	readonly runtime: InteractionRuntime;
	readonly history: DocumentHistory;
	readonly document: ModelDocument;
	readonly geometry: SpatialPickGeometry | null;
	readonly asideExtra?: ReactNode;
	readonly archivedBoxLayouts?: readonly ArchivedBoxLayout[];
	readonly onArchiveCommittedBox?: (layout: ArchivedBoxLayout) => void;
	/** @emoji 🔁 When host bumps this positive counter for the same preset, `cancel()` then `start` without remounting GL. */
	readonly sessionRestartNonce?: number;
}

/** @emoji 🪩 Full spatial REPL: canvas, interaction palette, history controls, last response. */
export function InteractionRepl({
	presets,
	interactionId,
	spec,
	onInteractionId,
	runtime: rt,
	history,
	document: documentModel,
	geometry,
	asideExtra,
	archivedBoxLayouts = [],
	onArchiveCommittedBox,
	sessionRestartNonce = 0,
}: InteractionReplProps): ReactNode {
	const snapshot = useInteractionSnapshot(rt);
	const histUi = useReplHistoryState(rt, spec, history);
	const mergedDisplay = useMemo(
		() => mergeDisplayWithArchivedBoxes(snapshot.display, archivedBoxLayouts),
		[snapshot.display, archivedBoxLayouts],
	);
	const [lastCommitLine, setLastCommitLine] = useState<string | null>(null);
	const [cmdLine, setCmdLine] = useState("");
	const [suggestOpen, setSuggestOpen] = useState(true);
	const [activeIndex, setActiveIndex] = useState(0);
	const [pickKindToggles, setPickKindToggles] = useState<Record<SpatialPickTargetKind, boolean>>(() =>
		defaultSpatialPickKindToggles(),
	);
	const [selectionMenu, setSelectionMenu] = useState<SpatialSelectionRequest | null>(null);
	const [hoveredPickKey, setHoveredPickKey] = useState<string | null>(null);
	const [selectedPickKey, setSelectedPickKey] = useState<string | null>(null);
	const cmdRef = useRef<HTMLInputElement>(null);
	const setCmdLineRef = useRef(setCmdLine);
	useEffect(() => {
		setCmdLineRef.current = setCmdLine;
	}, [setCmdLine]);

	useEffect(() => {
		const snap = rt.getSnapshot();
		const initial = spec.machine.initial;
		if (snap.state !== initial) return;
		const stDef = spec.machine.states.find((s) => s.name === snap.state);
		if (!stDef?.on?.some((h) => h.event === "start")) return;
		void rt.send({ kind: "start", modifiers: {} });
	}, [rt, spec]);

	useEffect(() => {
		if (sessionRestartNonce <= 0) return;
		rt.cancel();
		void rt.send({ kind: "start", modifiers: {} });
	}, [sessionRestartNonce, rt]);

	useEffect(() => {
		setSelectionMenu(null);
		setHoveredPickKey(null);
	}, [geometry, snapshot.state]);

	useEffect(() => {
		setSelectedPickKey(null);
	}, [geometry, interactionId]);

	const activeSelectionAccept = useMemo(() => rt.listActiveSelectionAccept(), [rt, snapshot.state]);

	const dispatchSelectionTarget = useCallback(
		(target: SpatialPickTarget, modifiers: InteractionEvent["modifiers"] = {}) => {
			setSelectionMenu(null);
			setHoveredPickKey(null);
			setSelectedPickKey(spatialPickTargetKey(target));
			void rt.send({
				kind: "selection.changed",
				targets: [{ kind: target.kind as TopologyEntityKind, id: target.id, editable: true }],
				modifiers,
			});
		},
		[rt],
	);

	const onSelectionRequest = useCallback(
		(request: SpatialSelectionRequest) => {
			if (request.targets.length === 1) {
				dispatchSelectionTarget(request.targets[0]!, request.modifiers);
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
				const accept = rt.listActiveSelectionAccept() as readonly TopologyEntityKind[];
				if (snapEv && accept.length > 0 && accept.includes(snapEv.kind as TopologyEntityKind)) {
					void rt.send({
						kind: "selection.changed",
						targets: [{ kind: snapEv.kind as TopologyEntityKind, id: snapEv.id, editable: true }],
						modifiers: (ev as { modifiers?: Record<string, unknown> }).modifiers ?? {},
					});
					return;
				}
			}
			if (ev.kind === "pointer.down" || ev.kind === "pointer.move") void rt.send(ev);
		},
		[rt],
	);

	const onCommit = useCallback(async () => {
		const res = await rt.commit();
		if (res.ok && !isEmptyTopologyDiff(res.diff) && onArchiveCommittedBox) {
			const raw = res.archiveContext;
			const box = raw ? tryArchivedBoxFromContext(raw) : null;
			if (box) onArchiveCommittedBox(box);
		}
		if (res.ok && res.data != null) {
			setLastCommitLine(`data: ${JSON.stringify(res.data)}`);
			console.log("[DEBUG] commit response data", res.data);
		} else if (!isEmptyTopologyDiff(res.diff)) {
			const fc = res.diff.faces?.added?.length ?? 0;
			setLastCommitLine(`diff (e.g. faces.added count=${fc})`);
			console.log("[DEBUG] commit topology diff", res.diff);
		} else if (!res.ok) {
			setLastCommitLine(res.errors.map((e) => e.message).join("; ") || "commit failed");
		} else {
			setLastCommitLine("ok (empty diff, no data)");
			console.log("[DEBUG] commit ok empty", res);
		}
	}, [rt, onArchiveCommittedBox]);

	const dispatchTransition = useCallback(
		(row: InteractionKeybindRow) => {
			const ev = replBuildDispatchEvent(row, { interactionId: spec.id, topo: documentModel.topology });
			if (ev) void rt.send(ev);
		},
		[rt, spec.id, documentModel.topology],
	);

	const allSuggestions = useMemo((): ReplSuggestion[] => {
		const st = snapshot.state;
		const rows = listKeyedInteractionTransitions(spec, st);
		const out: ReplSuggestion[] = [];
		for (const p of presets) {
			out.push({
				kind: "preset",
				key: p.key,
				label: p.label,
				detail: p.id,
				interactionId: p.id,
				onRun: () => onInteractionId(p.id),
			});
		}
		for (const row of rows) {
			out.push({
				kind: "transition",
				key: row.key,
				label: row.label,
				detail: row.eventKind,
				transition: row,
				onRun: () => dispatchTransition(row),
			});
		}
		out.push({ kind: "host", key: "m", label: "Commit solid", detail: "host", onRun: () => void onCommit() });
		out.push({ kind: "host", key: "r", label: "Undo", detail: "host", onRun: () => rt.undo() });
		out.push({ kind: "host", key: "y", label: "Redo", detail: "host", onRun: () => rt.redo() });
		return out;
	}, [presets, spec, snapshot.state, onInteractionId, dispatchTransition, onCommit, rt]);

	const filtered = useMemo(() => replPaletteRows(cmdLine, allSuggestions), [cmdLine, allSuggestions]);

	useEffect(() => {
		setActiveIndex((i) => (filtered.length ? Math.min(i, filtered.length - 1) : 0));
	}, [filtered.length, cmdLine]);

	const runSuggestion = useCallback((s: ReplSuggestion) => {
		s.onRun();
		setCmdLine("");
		setSuggestOpen(true);
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
		const presetHit = resolveSpatialInteractionPresetKey(raw);
		if (presetHit) {
			onInteractionId(presetHit.id);
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
		if (raw.toLowerCase() === "m") {
			void onCommit();
			setCmdLine("");
			return true;
		}
		if (raw.toLowerCase() === "r") {
			rt.undo();
			setCmdLine("");
			return true;
		}
		if (raw.toLowerCase() === "y") {
			rt.redo();
			setCmdLine("");
			return true;
		}
		return false;
	}, [cmdLine, spec, rt, dispatchTransition, onInteractionId, onCommit]);

	const onInputKeyDown = useCallback(
		(e: KeyboardEvent<HTMLInputElement>) => {
			if (e.key === "Escape") {
				e.preventDefault();
				setCmdLine("");
				setSuggestOpen(true);
				return;
			}
			if (e.key === "ArrowDown" && filtered.length) {
				e.preventDefault();
				setSuggestOpen(true);
				setActiveIndex((i) => (i + 1) % filtered.length);
				return;
			}
			if (e.key === "ArrowUp" && filtered.length) {
				e.preventDefault();
				setSuggestOpen(true);
				setActiveIndex((i) => (i - 1 + filtered.length) % filtered.length);
				return;
			}
			if (e.key === "Tab" && filtered.length) {
				e.preventDefault();
				setSuggestOpen(true);
				runSuggestion(filtered[activeIndex]!);
				return;
			}
			if (e.key === "Enter") {
				e.preventDefault();
				if (trySubmitLine()) return;
				if (filtered.length) runSuggestion(filtered[activeIndex]!);
				return;
			}
		},
		[filtered, activeIndex, runSuggestion, trySubmitLine],
	);

	useEffect(() => {
		const onWinCapture = (e: KeyboardEvent) => {
			if (e.defaultPrevented || e.isComposing) return;
			const t = e.target;
			const one = e.key.length === 1 ? e.key : "";
			if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "z") {
				if (replIsTextTypingTarget(t) && t !== cmdRef.current) return;
				e.preventDefault();
				e.stopPropagation();
				if (e.shiftKey) rt.redo();
				else rt.undo();
				return;
			}
			if (one) {
				const fac = replPresetFromShortcutKey(one, presets);
				if (fac) {
					if (replIsTextTypingTarget(t) && t !== cmdRef.current) return;
					e.preventDefault();
					e.stopPropagation();
					onInteractionId(fac.id);
					setCmdLineRef.current("");
					return;
				}
			}
			if (replIsTextTypingTarget(t) && t !== cmdRef.current) return;
			if (e.key === "m" || e.key === "M") {
				e.preventDefault();
				e.stopPropagation();
				void onCommit();
				return;
			}
			if (e.key === "r" || e.key === "R") {
				e.preventDefault();
				e.stopPropagation();
				rt.undo();
				return;
			}
			if (e.key === "y" || e.key === "Y") {
				e.preventDefault();
				e.stopPropagation();
				rt.redo();
			}
		};
		window.addEventListener("keydown", onWinCapture, true);
		return () => window.removeEventListener("keydown", onWinCapture, true);
	}, [rt, onCommit, presets, onInteractionId]);

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

	const kindLabel = (k: ReplSuggestKind) => (k === "preset" ? "Preset" : k === "transition" ? "Transition" : "Host");

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
						displayModel={mergedDisplay}
						selectionAccept={activeSelectionAccept}
						pickKindToggles={pickKindToggles}
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
									onPointerEnter={() => setHoveredPickKey(key)}
									onPointerLeave={() => setHoveredPickKey(null)}
									onPointerDown={(e) => {
										e.preventDefault();
										e.stopPropagation();
										dispatchSelectionTarget(target, selectionMenu.modifiers);
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
				{asideExtra}
				<div style={{ display: "flex", flexDirection: "column", gap: 6, fontSize: 12 }}>
					<span>Selectable kinds</span>
					<div style={{ display: "flex", flexWrap: "wrap", gap: 6 }}>
						{SPATIAL_PICK_TARGET_KINDS.map((kind) => {
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
										background: pickKindToggles[kind] ? "#1a2638" : "#12121c",
									}}
								>
									<input
										type="checkbox"
										checked={pickKindToggles[kind]}
										onChange={(e) => {
											const checked = e.target.checked;
											setPickKindToggles((prev) => ({ ...prev, [kind]: checked }));
											setSelectionMenu(null);
											setHoveredPickKey(null);
										}}
									/>
									{kind}
								</label>
							);
						})}
					</div>
				</div>
				<div style={{ fontSize: 12, opacity: 0.85 }}>
					Interaction <code>{interactionId}</code> · state <code>{snapshot.state}</code> · rev {snapshot.revision}
				</div>
				<div style={{ fontSize: 12 }}>
					Can commit {String(snapshot.capabilities.canCommit)} · undo {String(snapshot.capabilities.canUndo)} · redo{" "}
					{String(snapshot.capabilities.canRedo)}
				</div>
				<div style={{ display: "flex", gap: 8, flexWrap: "wrap" }}>
					<button
						type="button"
						disabled={!histUi.canUndo}
						onClick={() => rt.undo()}
						style={{ padding: "6px 10px", borderRadius: 6, opacity: histUi.canUndo ? 1 : 0.45 }}
					>
						Undo {histUi.undoLabel ? `(${histUi.undoLabel})` : ""}
					</button>
					<button
						type="button"
						disabled={!histUi.canRedo}
						onClick={() => rt.redo()}
						style={{ padding: "6px 10px", borderRadius: 6, opacity: histUi.canRedo ? 1 : 0.45 }}
					>
						Redo {histUi.redoLabel ? `(${histUi.redoLabel})` : ""}
					</button>
				</div>
				<div style={{ position: "relative" }}>
					<label style={{ display: "flex", flexDirection: "column", gap: 6, fontSize: 12 }}>
						<span>Interaction (presets first in palette; Tab/Enter run highlighted)</span>
						<input
							ref={cmdRef}
							type="text"
							autoComplete="off"
							value={cmdLine}
							onChange={(e) => {
								setCmdLine(e.target.value);
								setSuggestOpen(true);
							}}
							onFocus={() => setSuggestOpen(true)}
							onBlur={() => {
								window.setTimeout(() => setSuggestOpen(false), 120);
							}}
							onKeyDown={onInputKeyDown}
							placeholder="Filter or type an interaction…"
							style={{
								width: "100%",
								boxSizing: "border-box",
								padding: 8,
								borderRadius: 6,
								background: "#0e0e16",
								color: "#e8e8f0",
								border: "1px solid #2a2a3a",
							}}
						/>
					</label>
					{suggestOpen && filtered.length ? (
						<div
							onPointerDown={(e) => e.stopPropagation()}
							style={{
								position: "absolute",
								left: 0,
								right: 0,
								top: "100%",
								marginTop: 4,
								maxHeight: 280,
								overflowY: "auto",
								background: "#0c0c14",
								border: "1px solid #3a3a55",
								borderRadius: 6,
								zIndex: 10050,
								boxShadow: "0 8px 24px rgba(0,0,0,0.45)",
							}}
						>
							{filtered.map((s, idx) => (
								<button
									key={`${s.kind}-${s.key}-${s.detail}-${idx}`}
									type="button"
									onPointerDown={(e) => {
										e.preventDefault();
										e.stopPropagation();
										runSuggestion(s);
									}}
									style={{
										display: "block",
										width: "100%",
										textAlign: "left",
										padding: "6px 8px",
										border: "none",
										borderBottom: "1px solid #1e1e2e",
										background: idx === activeIndex ? "#1f2f4a" : "transparent",
										color: "#e8e8f0",
										cursor: "pointer",
										fontSize: 12,
									}}
									onMouseEnter={() => setActiveIndex(idx)}
								>
									<span style={{ opacity: 0.65 }}>{kindLabel(s.kind)}</span>{" "}
									{replPresentationWithUnderlinedKey(s.key, s.label)}
									<span style={{ opacity: 0.55, marginLeft: 6 }}>{s.detail}</span>
								</button>
							))}
						</div>
					) : null}
				</div>
				<div style={{ fontSize: 11, opacity: 0.75, lineHeight: 1.45 }}>
					Keys <u>q</u>/<u>j</u>/<u>k</u>/<u>d</u>/<u>a</u> switch interaction preset from anywhere (capture phase, clears the filter). <u>m</u> commits, <u>r</u> undoes, <u>y</u> redoes,{" "}
					<code>Ctrl+Z</code>/<code>Ctrl+Shift+Z</code> (except while typing in other text fields). Value-style interactions: <code>h 2.5</code>, <code>n 0.4</code>,{" "}
					<code>w 2 1.5</code>.
				</div>
				{lastCommitLine ? <div style={{ fontSize: 12 }}>Last commit: {lastCommitLine}</div> : null}
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
				point: [1, 2, 3],
				modifiers: { shift: true },
				snap: { kind: "vertex", id: "v0", point: [1, 2, 3] },
				selection: { kind: "vertex", id: "v0" },
			});
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
	});
}
// #endregion 🧪Tests

export type { MeshPreview };
