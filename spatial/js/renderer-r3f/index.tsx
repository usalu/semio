// #region 🧲Header
/** @emoji 🎬 `@spatial/js-renderer-r3f` — R3F `FactoryDisplay`, ground picking, `FactoryCanvas`, and snapshot hook. See `spatial/fixtures/factory.json`. */
// #endregion 🧲Header

// #region 📥Imports
import { Line, OrbitControls, Text } from "@react-three/drei";
import { Canvas, type ThreeEvent } from "@react-three/fiber";
import { useEffect, useMemo, useState, type ReactNode } from "react";
import * as THREE from "three";
import type {
	DisplayItem,
	DisplayModel,
	FactoryRuntime,
	FactorySnapshot,
	MeshPreview,
	Vec3,
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

// #region 🖼️DisplayPrimitives
function BoxPreviewItem({ item }: { readonly item: DisplayItem }): ReactNode {
	const p = item.params;
	const edgeGeo = useMemo(() => new THREE.EdgesGeometry(new THREE.BoxGeometry(1, 1, 1)), []);
	if (!p) return null;
	const a = readVec3(p.cornerA);
	const b = readVec3(p.cornerB);
	const h = readNumber(p.height);
	if (!a || !b || h === null) return null;
	const { position, scale } = computeBoxPreviewLayout(a, b, h);
	return (
		<group position={position} scale={scale}>
			<mesh raycast={raycastNone}>
				<boxGeometry args={[1, 1, 1]} />
				<meshStandardMaterial color="#6a8cff" transparent opacity={0.32} depthWrite={false} />
			</mesh>
			<lineSegments raycast={raycastNone} geometry={edgeGeo}>
				<lineBasicMaterial color="#dde6ff" transparent opacity={0.55} />
			</lineSegments>
		</group>
	);
}

function PointItem({ item }: { readonly item: DisplayItem }): ReactNode {
	const pos = readVec3(item.params?.position);
	if (!pos) return null;
	return (
		<mesh position={pos} raycast={raycastNone}>
			<sphereGeometry args={[0.06, 16, 16]} />
			<meshStandardMaterial color="#ffcc66" emissive="#553300" emissiveIntensity={0.35} />
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
	const span = 2;
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
/** @emoji 🖱️ Ground plane hit-test for `pointer.down` factory events (Z≈0 workspace). */
export interface GroundPickPlaneProps {
	readonly planeZ?: number;
	readonly enabled?: boolean;
	readonly onPick?: (point: Vec3) => void;
}

export function GroundPickPlane({ planeZ = 0, enabled = true, onPick }: GroundPickPlaneProps): ReactNode {
	const onPointerDown = (e: ThreeEvent<PointerEvent>) => {
		if (!enabled || !onPick) return;
		e.stopPropagation();
		const p = e.point;
		onPick([p.x, p.y, planeZ] as unknown as Vec3);
	};
	return (
		<mesh rotation={[-Math.PI / 2, 0, 0]} position={[0, 0, planeZ]} onPointerDown={onPointerDown}>
			<planeGeometry args={[120, 120]} />
			<meshBasicMaterial transparent opacity={0.12} color="#6688ff" side={THREE.DoubleSide} />
		</mesh>
	);
}
// #endregion 🖱️Interaction

// #region 🧊CommittedMesh
function TessellatedCommitMesh({ mesh: preview }: { readonly mesh: MeshPreview }): ReactNode {
	const geom = useMemo(() => {
		const g = new THREE.BufferGeometry();
		g.setAttribute("position", new THREE.BufferAttribute(preview.positions, 3));
		g.setIndex(preview.indices);
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
/** @emoji 🪝 Subscribes to `FactoryRuntime` revision updates for React hosts. */
export function useFactorySnapshot(rt: FactoryRuntime): FactorySnapshot {
	const [snap, setSnap] = useState(() => rt.getSnapshot());
	useEffect(() => rt.subscribe(() => setSnap(rt.getSnapshot())), [rt]);
	return snap;
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
	readonly onGroundPick?: (point: Vec3) => void;
	readonly pickEnabled?: boolean;
	readonly committedMesh?: MeshPreview | null;
}

/** @emoji 🪩 Lights, orbit controls, ground picking, factory overlays, optional committed mesh. */
export function FactorySpatialView({
	snapshot,
	onGroundPick,
	pickEnabled = true,
	committedMesh,
}: FactorySpatialViewProps): ReactNode {
	const gridHelper = useMemo(() => {
		const g = new THREE.GridHelper(40, 40, 0x3a3a55, 0x1c1c28);
		g.position.set(0, 0.002, 0);
		return g;
	}, []);
	return (
		<>
			<ambientLight intensity={0.45} />
			<directionalLight position={[12, 18, 10]} intensity={1.1} />
			<OrbitControls makeDefault />
			<primitive object={gridHelper} />
			<GroundPickPlane enabled={pickEnabled} onPick={onGroundPick} />
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
}
// #endregion 🧪Tests

export type { MeshPreview };
// #region 🧲Header
/** @emoji 🧭 `@spatial/js-renderer-r3f` — React Three Fiber bindings for the spatial factory runtime. */
// #endregion 🧲Header

// #region 📥Imports
import { Canvas, type ThreeEvent } from "@react-three/fiber";
import React, { useCallback, useEffect, useMemo, useSyncExternalStore } from "react";
import * as THREE from "three";
import {
	buildBoxFactorySpec,
	createFactoryRuntime,
	type DisplayItem,
	type FactoryEvent,
	type FactoryRuntime,
	type FactoryRuntimeOptions,
	type FactorySnapshot,
	type FactorySpec,
	InMemoryKernel,
	type MeshPreview as CoreMeshPreview,
	TopologyGraph,
	type Vec3,
} from "@spatial/js-core";
// #endregion 📥Imports

// #region 🧾Types
/** @emoji 🖼️ Mesh preview payload re-exported for play/runtime integrations. */
export type MeshPreview = CoreMeshPreview;

/** @emoji 🎯 Ground-pick handler used by the interaction plane. */
export type GroundPickHandler = (point: Vec3) => void;
// #endregion 🧾Types

// #region 🎮InteractionAdapter
/** @emoji 🎮 Converts R3F pointer events into spatial factory events with Three.js world coordinates. */
export function createR3FInteractionAdapter() {
	const toPoint = (event: ThreeEvent<PointerEvent>): Vec3 => [event.point.x, event.point.y, event.point.z];
	const modifiers = (event: ThreeEvent<PointerEvent>) => ({
		alt: event.altKey,
		ctrl: event.ctrlKey,
		meta: event.metaKey,
		shift: event.shiftKey,
	});
	return {
		pointerMove: (event: ThreeEvent<PointerEvent>): FactoryEvent => ({
			kind: "pointer.move",
			point: toPoint(event),
			modifiers: modifiers(event),
		}),
		pointerDown: (event: ThreeEvent<PointerEvent>): FactoryEvent => ({
			kind: "pointer.down",
			point: toPoint(event),
			modifiers: modifiers(event),
		}),
	};
}
// #endregion 🎮InteractionAdapter

// #region 🖼️DisplayAdapter
function itemPoint(item: DisplayItem, key: string, fallback: Vec3): Vec3 {
	const value = item.params?.[key];
	if (Array.isArray(value) && value.length === 3 && value.every((v) => typeof v === "number")) return value as unknown as Vec3;
	return fallback;
}

function numberParam(item: DisplayItem, key: string, fallback: number): number {
	const value = item.params?.[key];
	return typeof value === "number" && Number.isFinite(value) ? value : fallback;
}

function BoxPreview({ item }: { readonly item: DisplayItem }) {
	const a = itemPoint(item, "cornerA", [0, 0, 0]);
	const b = itemPoint(item, "cornerB", [1, 1, 0]);
	const height = Math.max(numberParam(item, "height", 0.05), 0.05);
	const sx = Math.max(Math.abs(b[0] - a[0]), 0.05);
	const sy = Math.max(Math.abs(b[1] - a[1]), 0.05);
	const sz = height;
	const px = (a[0] + b[0]) / 2;
	const py = (a[1] + b[1]) / 2;
	const pz = Math.min(a[2], b[2]) + height / 2;
	return (
		<mesh position={[px, py, pz]} userData={{ spatialDisplayKind: item.kind, spatialDisplayId: item.id }}>
			<boxGeometry args={[sx, sy, sz]} />
			<meshStandardMaterial color="#5eead4" transparent opacity={0.38} />
		</mesh>
	);
}

function PointItem({ item }: { readonly item: DisplayItem }) {
	const point = itemPoint(item, "point", [0, 0, 0]);
	return (
		<mesh position={point} userData={{ spatialDisplayKind: item.kind, spatialDisplayId: item.id }}>
			<sphereGeometry args={[0.055, 12, 12]} />
			<meshStandardMaterial color="#facc15" />
		</mesh>
	);
}

function MeshItem({ meshPreview, id }: { readonly meshPreview: MeshPreview; readonly id: string }) {
	const geometry = useMemo(() => {
		const g = new THREE.BufferGeometry();
		g.setAttribute("position", new THREE.BufferAttribute(meshPreview.positions, 3));
		g.setIndex(new THREE.BufferAttribute(meshPreview.indices, 1));
		if (meshPreview.normals) g.setAttribute("normal", new THREE.BufferAttribute(meshPreview.normals, 3));
		else g.computeVertexNormals();
		return g;
	}, [meshPreview]);
	useEffect(() => () => geometry.dispose(), [geometry]);
	return (
		<mesh geometry={geometry} userData={{ spatialDisplayKind: "mesh", spatialDisplayId: id }}>
			<meshStandardMaterial color="#93c5fd" roughness={0.65} metalness={0.05} />
		</mesh>
	);
}

/** @emoji 🖼️ Renders the renderer-neutral `DisplayModel` primitive subset used by the first spatial runtime. */
export function FactoryDisplay({
	snapshot,
	committedMesh,
}: {
	readonly snapshot: FactorySnapshot;
	readonly committedMesh?: MeshPreview | null;
}) {
	return (
		<group>
			{snapshot.display.items.map((item) => {
				if (item.kind === "box-preview") return <BoxPreview key={item.id} item={item} />;
				if (item.kind === "point") return <PointItem key={item.id} item={item} />;
				return null;
			})}
			{committedMesh ? <MeshItem id="committed-cell" meshPreview={committedMesh} /> : null}
		</group>
	);
}
// #endregion 🖼️DisplayAdapter

// #region ⚛️Hooks
/** @emoji ⚛️ Creates a stable factory runtime for React consumers. */
export function useFactoryRuntime(spec: FactorySpec, opts: FactoryRuntimeOptions): FactoryRuntime {
	return useMemo(() => createFactoryRuntime(spec, opts), [spec, opts]);
}

/** @emoji ⚛️ Subscribes to a `FactoryRuntime` using React's external-store contract. */
export function useFactorySnapshot(runtime: FactoryRuntime): FactorySnapshot {
	return useSyncExternalStore(
		(listener) => runtime.subscribe(listener),
		() => runtime.getSnapshot(),
		() => runtime.getSnapshot(),
	);
}

/** @emoji ⚛️ Canvas shell with lights and camera defaults for spatial factory previews. */
export function FactoryCanvas({ children }: { readonly children: React.ReactNode }) {
	return (
		<Canvas camera={{ position: [4, -6, 4], fov: 45 }} style={{ width: "100%", height: "100%", background: "#0f172a" }}>
			<ambientLight intensity={0.65} />
			<directionalLight position={[3, -4, 6]} intensity={1.1} />
			<gridHelper args={[12, 12, "#334155", "#1e293b"]} rotation={[Math.PI / 2, 0, 0]} />
			{children}
		</Canvas>
	);
}

/** @emoji ⚛️ Invisible ground plane that forwards pick points into the factory runtime. */
export function FactoryInteractionLayer({
	onGroundPick,
	pickEnabled,
}: {
	readonly onGroundPick: GroundPickHandler;
	readonly pickEnabled: boolean;
}) {
	const adapter = useMemo(() => createR3FInteractionAdapter(), []);
	const onPointerDown = useCallback(
		(event: ThreeEvent<PointerEvent>) => {
			if (!pickEnabled) return;
			event.stopPropagation();
			const factoryEvent = adapter.pointerDown(event);
			onGroundPick(factoryEvent.point as Vec3);
		},
		[adapter, onGroundPick, pickEnabled],
	);
	return (
		<mesh position={[0, 0, 0]} onPointerDown={onPointerDown} visible={false}>
			<planeGeometry args={[100, 100]} />
			<meshBasicMaterial transparent opacity={0} />
		</mesh>
	);
}

/** @emoji ⚛️ Complete R3F factory view used by the spatial play demo. */
export function FactorySpatialView({
	snapshot,
	onGroundPick,
	pickEnabled,
	committedMesh,
}: {
	readonly snapshot: FactorySnapshot;
	readonly onGroundPick: GroundPickHandler;
	readonly pickEnabled: boolean;
	readonly committedMesh?: MeshPreview | null;
}) {
	return (
		<>
			<FactoryDisplay snapshot={snapshot} committedMesh={committedMesh} />
			<FactoryInteractionLayer onGroundPick={onGroundPick} pickEnabled={pickEnabled} />
		</>
	);
}
// #endregion ⚛️Hooks

// #region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

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
	});

	describe("@spatial/js-renderer-r3f runtime hook prerequisites", () => {
		it("creates a snapshot for the box factory with the in-memory kernel", () => {
			const spec = buildBoxFactorySpec();
			const runtime = createFactoryRuntime(spec, {
				kernel: new InMemoryKernel(),
				document: { topology: new TopologyGraph(), nodes: [] },
			});
			const snapshot = runtime.getSnapshot();
			expect(snapshot.factoryId).toBe(spec.id);
			expect(snapshot.state).toBe(spec.machine.initial);
			expect(snapshot.display.items.length).toBeGreaterThanOrEqual(0);
		});
	});
}
// #endregion 🧪Tests
