// #region 🧲Header
/** @emoji 🎬 `@spatial/js-renderer-r3f` — R3F `FactoryDisplay`, ground picking, interaction adapter, `FactoryCanvas`, and snapshot hooks. See `spatial/fixtures/factory.json`. */
// #endregion 🧲Header

// #region 📥Imports
import { Line, OrbitControls, Text } from "@react-three/drei";
import { Canvas, type ThreeEvent } from "@react-three/fiber";
import { useMemo, useSyncExternalStore, type ReactNode } from "react";
import { MOUSE } from "three";
import * as THREE from "three";
import {
	buildBoxCommandSpec as buildBoxFactorySpec,
	cellRef,
	createCommandRuntime as createFactoryRuntime,
	TopologyGraph,
	type DisplayItem,
	type DisplayModel,
	type EdgeRecord,
	type FaceRecord,
	type CommandEvent as FactoryEvent,
	type CommandRuntime as FactoryRuntime,
	type CommandRuntimeOptions as FactoryRuntimeOptions,
	type CommandSnapshot as FactorySnapshot,
	type CommandSpec as FactorySpec,
	type KernelAdapter,
	type MeshPreview,
	type TopologyGraphJson,
	type Vec3,
	type VertexRecord,
	type WireRecord,
} from "@spatial/js-core";
// #endregion 📥Imports

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
export type FactoryInteractionKind = "pointer.down" | "pointer.move";

export interface FactoryInteractionTarget {
	readonly kind: "vertex" | "edge" | "face" | "cell" | "cellComplex" | "cluster";
	readonly id: string;
	readonly point: Vec3;
	readonly points?: readonly Vec3[];
}

export type FactoryInteractionGeometry = TopologyGraph | TopologyGraphJson;

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

function topologyAllVertexPoints(geometry: FactoryInteractionGeometry): readonly Vec3[] {
	return topologyRecords(geometry.vertices).map((vertex) => vertex.position);
}

/** @emoji 🧲 Builds renderer-side snap/select targets from optional factory topology geometry. */
export function createFactoryInteractionTargets(geometry: FactoryInteractionGeometry | null | undefined): readonly FactoryInteractionTarget[] {
	if (!geometry) return [];
	const targets: FactoryInteractionTarget[] = [];
	for (const vertex of topologyRecords(geometry.vertices)) {
		targets.push({ kind: "vertex", id: vertex.id, point: vertex.position });
	}
	for (const edge of topologyRecords(geometry.edges)) {
		const points = topologyEdgePoints(geometry.vertices, edge);
		const point = topologyPointCentroid(points);
		if (point) targets.push({ kind: "edge", id: edge.id, point, points });
	}
	for (const face of topologyRecords(geometry.faces)) {
		const points = topologyFacePoints(geometry.vertices, geometry.edges, geometry.wires, face);
		const point = topologyPointCentroid(points);
		if (point) targets.push({ kind: "face", id: face.id, point, points });
	}
	const all = topologyAllVertexPoints(geometry);
	const allCenter = topologyPointCentroid(all);
	for (const cell of topologyRecords(geometry.cells)) {
		if (allCenter) targets.push({ kind: "cell", id: cell.id, point: allCenter, points: all });
	}
	for (const complex of topologyRecords(geometry.cellComplexes)) {
		if (allCenter) targets.push({ kind: "cellComplex", id: complex.id, point: allCenter, points: all });
	}
	for (const cluster of topologyRecords(geometry.clusters)) {
		if (allCenter) targets.push({ kind: "cluster", id: cluster.id, point: allCenter, points: all });
	}
	return targets;
}

/** @emoji 🧲 Creates a statechart event carrying snapped point plus selected topology metadata. */
export function createFactoryInteractionEvent(
	kind: FactoryInteractionKind,
	point: Vec3,
	target: FactoryInteractionTarget | null,
	modifiers: FactoryEvent["modifiers"] = {},
): FactoryEvent {
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
	return (
		<group position={position} scale={scale}>
			<mesh raycast={raycastNone}>
				<boxGeometry args={[1, 1, 1]} />
				<meshStandardMaterial
					color="#7ab0ff"
					emissive="#102a66"
					emissiveIntensity={0.35}
					transparent
					opacity={0.52}
					depthWrite={false}
				/>
			</mesh>
			<lineSegments raycast={raycastNone} geometry={edgeGeo}>
				<lineBasicMaterial color="#ffffff" transparent opacity={0.85} />
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
export function FactoryDisplay({ model }: { readonly model: DisplayModel }): ReactNode {
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

/** @emoji 🎮 Maps R3F pointer events to `FactoryEvent` envelopes (point + modifiers). */
export function createR3FInteractionAdapter() {
	const toPoint = (event: ThreeEvent<PointerEvent>): Vec3 => [event.point.x, event.point.y, event.point.z];
	return {
		pointerMove: (event: ThreeEvent<PointerEvent>): FactoryEvent => ({
			kind: "pointer.move",
			point: toPoint(event),
			modifiers: pointerModifiers(event),
		}),
		pointerDown: (event: ThreeEvent<PointerEvent>): FactoryEvent => ({
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

function FactoryInteractionTargetNode({
	target,
	onFactoryEvent,
	onPick,
	onPointerMove,
	pointerMoveEnabled,
}: {
	readonly target: FactoryInteractionTarget;
	readonly onFactoryEvent?: (event: FactoryEvent) => void;
	readonly onPick?: (point: Vec3, event: FactoryEvent) => void;
	readonly onPointerMove?: (point: Vec3, event: FactoryEvent) => void;
	readonly pointerMoveEnabled: boolean;
}): ReactNode {
	const emit = (kind: FactoryInteractionKind, e: ThreeEvent<PointerEvent>) => {
		e.stopPropagation();
		const event = createFactoryInteractionEvent(kind, [e.point.x, e.point.y, e.point.z], target, pointerModifiers(e));
		onFactoryEvent?.(event);
		if (kind === "pointer.down") onPick?.(target.point, event);
		if (kind === "pointer.move" && pointerMoveEnabled) onPointerMove?.(target.point, event);
	};
	const onPointerDown = (e: ThreeEvent<PointerEvent>) => emit("pointer.down", e);
	const onPointerMoveH = (e: ThreeEvent<PointerEvent>) => {
		if (!pointerMoveEnabled) return;
		emit("pointer.move", e);
	};
	if (target.kind === "vertex") {
		return (
			<mesh position={target.point} onPointerDown={onPointerDown} onPointerMove={onPointerMoveH} renderOrder={4}>
				<sphereGeometry args={[0.085, 16, 16]} />
				<meshStandardMaterial color="#ffdf7a" emissive="#4a3000" emissiveIntensity={0.35} />
			</mesh>
		);
	}
	if (target.points && target.points.length >= 2 && target.kind === "edge") {
		return (
			<Line points={target.points.map((p) => [p[0], p[1], p[2]])} color="#ffd166" lineWidth={5} onPointerDown={onPointerDown} onPointerMove={onPointerMoveH} />
		);
	}
	const bounds = target.points ? targetBounds(target.points) : null;
	if (!bounds) return null;
	return (
		<mesh position={bounds.center} scale={bounds.size} onPointerDown={onPointerDown} onPointerMove={onPointerMoveH} renderOrder={1}>
			<boxGeometry args={[1, 1, 1]} />
			<meshStandardMaterial color="#f6c85f" transparent opacity={0.16} depthWrite={false} side={THREE.DoubleSide} />
		</mesh>
	);
}

/** @emoji 🧲 Renders optional factory geometry as pickable snap/select targets. */
export function FactoryInteractionGeometryLayer({
	geometry,
	onFactoryEvent,
	onPick,
	onPointerMove,
	pointerMoveEnabled = false,
}: {
	readonly geometry?: FactoryInteractionGeometry | null;
	readonly onFactoryEvent?: (event: FactoryEvent) => void;
	readonly onPick?: (point: Vec3, event: FactoryEvent) => void;
	readonly onPointerMove?: (point: Vec3, event: FactoryEvent) => void;
	readonly pointerMoveEnabled?: boolean;
}): ReactNode {
	const topoRevision =
		geometry && typeof geometry === "object" && "revision" in geometry
			? Number((geometry as { revision?: unknown }).revision)
			: 0;
	const targets = useMemo(() => createFactoryInteractionTargets(geometry), [geometry, topoRevision]);
	return (
		<group>
			{targets.map((target) => (
				<FactoryInteractionTargetNode
					key={`${target.kind}:${target.id}`}
					target={target}
					onFactoryEvent={onFactoryEvent}
					onPick={onPick}
					onPointerMove={onPointerMove}
					pointerMoveEnabled={pointerMoveEnabled}
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
/** @emoji 🪝 Memoized `createFactoryRuntime` for React hosts. */
export function useFactoryRuntime(spec: FactorySpec, opts: FactoryRuntimeOptions): FactoryRuntime {
	return useMemo(() => createFactoryRuntime(spec, opts), [spec, opts]);
}

/** @emoji 🪝 Subscribes to `FactoryRuntime` revision updates for React hosts. */
export function useFactorySnapshot(rt: FactoryRuntime): FactorySnapshot {
	return useSyncExternalStore(
		(cb) => rt.subscribe(cb),
		() => rt.getSnapshot(),
		() => rt.getSnapshot(),
	);
}
// #endregion 🪝Hooks

// #region 🪩Canvas
export interface FactoryCanvasProps {
	readonly children: ReactNode;
}

/** @emoji 🪩 Root `<Canvas>` preset for factory viewports. */
export function FactoryCanvas({ children }: FactoryCanvasProps): ReactNode {
	return (
		<Canvas style={{ height: "100%", width: "100%" }} camera={{ position: [10, 10, 8], fov: 45 }}>
			<color attach="background" args={["#080810"]} />
			{children}
		</Canvas>
	);
}

export interface FactorySpatialViewProps {
	readonly snapshot: FactorySnapshot;
	readonly onGroundPick?: (point: Vec3, event: FactoryEvent) => void;
	/** @emoji 🖱️ `pointer.move` hits ground (XY at fixed Z); height slab passes full 3D. */
	readonly onScenePointerMove?: (point: Vec3, event: FactoryEvent) => void;
	readonly onFactoryEvent?: (event: FactoryEvent) => void;
	readonly pickEnabled?: boolean;
	readonly committedMesh?: MeshPreview | null;
	readonly geometry?: FactoryInteractionGeometry | null;
}

/** @emoji 🪩 Lights, orbit controls, ground picking, factory overlays, optional committed mesh. */
export function FactorySpatialView({
	snapshot,
	onGroundPick,
	onScenePointerMove,
	onFactoryEvent,
	pickEnabled = true,
	committedMesh,
	geometry,
}: FactorySpatialViewProps): ReactNode {
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
		const event = createFactoryInteractionEvent("pointer.down", point, null);
		onFactoryEvent?.(event);
		onGroundPick?.(point, event);
	};
	const onScenePointerMoveEvent = (point: Vec3) => {
		const event = createFactoryInteractionEvent("pointer.move", point, null);
		onFactoryEvent?.(event);
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
			<FactoryInteractionGeometryLayer
				geometry={geometry}
				onFactoryEvent={onFactoryEvent}
				onPick={onGroundPick}
				onPointerMove={onScenePointerMove}
				pointerMoveEnabled={groundMoveOn || heightMoveOn || zRodMoveOn}
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
			<FactoryDisplay model={snapshot.display} />
			{committedMesh ? <TessellatedCommitMesh mesh={committedMesh} /> : null}
		</>
	);
}
// #endregion 🪩Canvas

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
		it("maps pointer event data into factory events", () => {
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
			const targets = createFactoryInteractionTargets({
				schema: "spatial.topology/v1",
				revision: 1,
				vertices: {
					v0: { id: "v0", position: [1, 2, 3] },
				},
				edges: {},
				wires: {},
				faces: {},
				shells: {},
				cells: {},
				cellComplexes: {},
				clusters: {},
			});
			expect(targets).toEqual([{ kind: "vertex", id: "v0", point: [1, 2, 3] }]);
			expect(createFactoryInteractionEvent("pointer.down", [9, 9, 9], targets[0]!, { shift: true })).toEqual({
				kind: "pointer.down",
				point: [1, 2, 3],
				modifiers: { shift: true },
				snap: { kind: "vertex", id: "v0", point: [1, 2, 3] },
				selection: { kind: "vertex", id: "v0" },
			});
		});
	});

	describe("@spatial/js-renderer-r3f runtime", () => {
		it("exposes an initial snapshot for the box factory with a stub kernel", () => {
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
			const spec = buildBoxFactorySpec();
			const runtime = createFactoryRuntime(spec, {
				kernel: new StubKernel(),
				document: { topology: new TopologyGraph(), nodes: [] },
			});
			const snapshot = runtime.getSnapshot();
			expect(snapshot.factoryId).toBe(spec.id);
			expect(snapshot.state).toBe(spec.machine.initial);
		});
	});
}
// #endregion 🧪Tests

export type { MeshPreview };
