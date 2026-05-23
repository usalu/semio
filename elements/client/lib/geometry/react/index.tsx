// #region 🧲Header
// 💻 elements/client/lib/geometry/react/index.tsx — Topologic R3F components for every core kind plus a canvas shell with selection and transform gumball support.
// #endregion 🧲Header

import { Line, OrbitControls, TransformControls } from "@react-three/drei";
import { Canvas } from "@react-three/fiber";
import {
	createContext,
	useCallback,
	useContext,
	useEffect,
	useMemo,
	useRef,
	useState,
	type CSSProperties,
	type ReactElement,
	type ReactNode,
} from "react";
import {
	BufferGeometry,
	DoubleSide,
	Float32BufferAttribute,
	Group,
	Quaternion,
	Vector3,
	type Object3D,
} from "three";

import {
	TOPOLOGIC_KINDS,
	type TopologicKind,
	centroid,
	collectDragAttachIds,
	collectContainerPickPoints,
	computePickBounds,
	TopologicCellComplexEntity,
	TopologicCellEntity,
	TopologicClusterEntity,
	TopologicEdgeEntity,
	type TopologicEntity,
	TopologicFaceEntity,
	type TopologicFixtureV1,
	TopologicShellEntity,
	TopologicTopologyEntity,
	type TopologicTransform,
	TopologicVertexEntity,
	TopologicWasmSession,
	TopologicWireEntity,
	ensureTopologicWasmLoaded,
	loadTopologicFixtureV1,
	normalizeTransform,
	edgeModelPoints,
	resolveEntityGroupTransform,
	type Vec3,
} from "../wasm/index.ts";

//#region 🔖Context
export type TopologicTransformMode = "translate" | "rotate" | "scale";

interface TopologicSceneValue {
	readonly session: TopologicWasmSession;
	readonly selectedId: string | null;
	readonly registerObject: (id: string, object: Group | null) => void;
	readonly objectById: ReadonlyMap<string, Group>;
	readonly onSelect?: (id: string | null) => void;
	readonly onTransformCommit?: (id: string, transform: TopologicTransform) => void;
	readonly transformMode: TopologicTransformMode;
	readonly selectableKinds?: Readonly<Partial<Record<TopologicKind, boolean>>>;
}

function disableRaycast(): null {
	return null;
}

function isEntitySelectable(scene: TopologicSceneValue, entityId: string): boolean {
	if (!scene.selectableKinds) return true;
	const entity = scene.session.getEntity(entityId);
	if (!entity) return false;
	return scene.selectableKinds[entity.kind] ?? true;
}

const TopologicSceneContext = createContext<TopologicSceneValue | null>(null);

function useTopologicScene(): TopologicSceneValue {
	const value = useContext(TopologicSceneContext);
	if (!value) throw new Error("TopologicSceneContext missing");
	return value;
}

function useIsSelected(id: string): boolean {
	return useTopologicScene().selectedId === id;
}
//#endregion 🔖Context

//#region 🔖Colors
function topologyColor(entity: { readonly style?: { readonly color?: string; readonly edgeColor?: string } }, fallback: string): string {
	return entity.style?.color ?? entity.style?.edgeColor ?? fallback;
}

function topologyOpacity(entity: { readonly style?: { readonly opacity?: number } }, fallback: number): number {
	return entity.style?.opacity ?? fallback;
}

function topologyLineWidth(entity: { readonly style?: { readonly lineWidth?: number } }, fallback: number): number {
	return entity.style?.lineWidth ?? fallback;
}

function edgePickRadius(entity: { readonly style?: { readonly lineWidth?: number } }, fallback: number): number {
	return Math.max(0.12, topologyLineWidth(entity, fallback) * 0.1);
}

export function edgePickSegments(points: readonly Vec3[]): ReadonlyArray<readonly [Vec3, Vec3]> {
	const segments: [Vec3, Vec3][] = [];
	for (let index = 0; index < points.length - 1; index += 1) {
		segments.push([points[index], points[index + 1]]);
	}
	return segments;
}

function useSegmentPose(start: Vec3, end: Vec3): {
	readonly position: Vec3;
	readonly quaternion: readonly [number, number, number, number];
	readonly length: number;
} {
	return useMemo(() => {
		const direction = new Vector3(end[0] - start[0], end[1] - start[1], end[2] - start[2]);
		const length = direction.length();
		const quaternion = new Quaternion();
		if (length > 1e-9) quaternion.setFromUnitVectors(new Vector3(0, 1, 0), direction.normalize());
		return {
			position: [(start[0] + end[0]) / 2, (start[1] + end[1]) / 2, (start[2] + end[2]) / 2],
			quaternion: [quaternion.x, quaternion.y, quaternion.z, quaternion.w],
			length,
		};
	}, [end, start]);
}

function TopologicEdgePickSegment(props: {
	readonly start: Vec3;
	readonly end: Vec3;
	readonly radius: number;
	readonly onSelect: () => void;
}): ReactElement | null {
	const pose = useSegmentPose(props.start, props.end);
	if (pose.length < 1e-9) return null;
	return (
		<mesh
			position={pose.position}
			quaternion={pose.quaternion}
			scale={[props.radius, pose.length, props.radius]}
			onPointerDown={(event) => {
				event.stopPropagation();
				props.onSelect();
			}}
		>
			<cylinderGeometry args={[1, 1, 1, 8]} />
			<meshBasicMaterial transparent opacity={0} depthWrite={false} />
		</mesh>
	);
}

function TopologicEdgePick(props: {
	readonly points: readonly Vec3[];
	readonly radius: number;
	readonly onSelect: () => void;
}): ReactElement {
	return (
		<>
			{edgePickSegments(props.points).map(([start, end]) => (
				<TopologicEdgePickSegment key={`${start.join(",")}-${end.join(",")}`} start={start} end={end} radius={props.radius} onSelect={props.onSelect} />
			))}
		</>
	);
}

function selectedColor(base: string, selected: boolean): string {
	return selected ? "#f97316" : base;
}
//#endregion 🔖Colors

//#region 🔖Groups
function transformProps(transform: TopologicTransform | undefined): {
	readonly position: Vec3;
	readonly quaternion: readonly [number, number, number, number];
	readonly scale: readonly [number, number, number];
} {
	const normalized = normalizeTransform(transform);
	const scale = typeof normalized.scale === "number" ? [normalized.scale, normalized.scale, normalized.scale] : normalized.scale;
	return {
		position: normalized.position,
		quaternion: normalized.rotation,
		scale,
	};
}

function TopologicGroup(props: {
	readonly entityId: string;
	readonly transform: TopologicTransform | undefined;
	readonly children: ReactNode;
}): ReactElement {
	const ref = useRef<Group>(null);
	const scene = useTopologicScene();
	const registerObject = scene.registerObject;
	const { position, quaternion, scale } = transformProps(props.transform);
	useEffect(() => {
		registerObject(props.entityId, ref.current);
		return () => registerObject(props.entityId, null);
	}, [props.entityId, registerObject]);
	return (
		<group
			ref={ref}
			position={position}
			quaternion={quaternion}
			scale={scale}
			onPointerDown={(event) => {
				if (!isEntitySelectable(scene, props.entityId)) return;
				event.stopPropagation();
				scene.onSelect?.(props.entityId);
			}}
		>
			{props.children}
		</group>
	);
}
//#endregion 🔖Groups

//#region 🔖Geometry
function offsetPoint(point: Vec3, anchor: Vec3): Vec3 {
	return [point[0] - anchor[0], point[1] - anchor[1], point[2] - anchor[2]];
}

function useFaceGeometry(vertices: readonly Vec3[], triangles: readonly number[]): BufferGeometry {
	const geometry = useMemo(() => {
		const next = new BufferGeometry();
		next.setAttribute("position", new Float32BufferAttribute(vertices.flat(), 3));
		next.setIndex([...triangles]);
		next.computeVertexNormals();
		return next;
	}, [triangles, vertices]);
	useEffect(() => () => geometry.dispose(), [geometry]);
	return geometry;
}
//#endregion 🔖Geometry

//#region 🔖Traversal
interface ResolvedTopologyEntry {
	readonly entity: TopologicEntity;
	readonly transform: TopologicTransform | undefined;
}

interface TopologyTraversalResult {
	readonly entries: readonly ResolvedTopologyEntry[];
	readonly revisitedIds: readonly string[];
}

export function collectSceneEntries(session: TopologicWasmSession): TopologyTraversalResult {
	const entries: ResolvedTopologyEntry[] = [];
	const visited = new Set<string>();
	const revisitedIds = new Set<string>();
	const visit = (entityId: string): void => {
		const entity = session.getEntity(entityId);
		if (!entity) return;
		if (visited.has(entity.id)) {
			revisitedIds.add(entity.id);
			return;
		}
		visited.add(entity.id);
		entries.push({ entity, transform: resolveEntityGroupTransform(session, entity, entity.transform) });
		for (const child of session.childrenOf(entity.id)) {
			visit(child.id);
		}
	};
	for (const rootId of session.fixture.roots) visit(rootId);
	return { entries, revisitedIds: [...revisitedIds] };
}
//#endregion 🔖Traversal

//#region 🔖Kinds
function TopologyAnchor(props: { readonly entityId: string; readonly transform: TopologicTransform | undefined }): ReactElement {
	return <TopologicGroup entityId={props.entityId} transform={props.transform} />;
}

function TopologicContainer(props: { readonly entityId: string; readonly transform?: TopologicTransform }): ReactElement {
	const scene = useTopologicScene();
	const bounds = useMemo(() => {
		const points = collectContainerPickPoints(scene.session, props.entityId);
		return computePickBounds(points);
	}, [props.entityId, scene.session]);
	const select = useCallback(() => scene.onSelect?.(props.entityId), [props.entityId, scene.onSelect]);
	if (!bounds || !isEntitySelectable(scene, props.entityId)) {
		return <TopologyAnchor entityId={props.entityId} transform={props.transform} />;
	}
	return (
		<TopologicGroup entityId={props.entityId} transform={props.transform}>
			<mesh
				onPointerDown={(event) => {
					event.stopPropagation();
					select();
				}}
			>
				<boxGeometry args={bounds.size} />
				<meshBasicMaterial transparent opacity={0} depthWrite={false} />
			</mesh>
		</TopologicGroup>
	);
}

export function Vertex(props: { readonly entity: TopologicVertexEntity; readonly transform?: TopologicTransform }): ReactElement {
	const scene = useTopologicScene();
	const selectable = isEntitySelectable(scene, props.entity.id);
	const selected = useIsSelected(props.entity.id);
	const color = selectedColor(topologyColor(props.entity, "#38bdf8"), selected);
	const radius = props.entity.radius ?? props.entity.style?.pointSize ?? 0.12;
	return (
		<TopologicGroup entityId={props.entity.id} transform={props.transform}>
			<mesh raycast={selectable ? undefined : disableRaycast}>
				<sphereGeometry args={[radius, 24, 24]} />
				<meshStandardMaterial color={color} transparent opacity={topologyOpacity(props.entity, 1)} />
			</mesh>
		</TopologicGroup>
	);
}

export function Edge(props: { readonly entity: TopologicEdgeEntity; readonly transform?: TopologicTransform }): ReactElement {
	const scene = useTopologicScene();
	const selected = useIsSelected(props.entity.id);
	const points = useMemo(() => edgeModelPoints(scene.session, props.entity), [props.entity, scene.session]);
	const anchor = useMemo(() => centroid(points), [points]);
	const localPoints = useMemo(() => points.map((point) => offsetPoint(point, anchor)), [anchor, points]);
	const pickRadius = edgePickRadius(props.entity, 2);
	const selectable = isEntitySelectable(scene, props.entity.id);
	const select = useCallback(() => scene.onSelect?.(props.entity.id), [props.entity.id, scene.onSelect]);
	return (
		<TopologicGroup entityId={props.entity.id} transform={props.transform}>
			<Line
				points={localPoints}
				color={selectedColor(topologyColor(props.entity, "#f8fafc"), selected)}
				lineWidth={topologyLineWidth(props.entity, 2)}
				raycast={disableRaycast}
			/>
			{selectable ? <TopologicEdgePick points={localPoints} radius={pickRadius} onSelect={select} /> : null}
		</TopologicGroup>
	);
}

export function Wire(props: { readonly entity: TopologicWireEntity; readonly transform?: TopologicTransform }): ReactElement {
	return <TopologyAnchor entityId={props.entity.id} transform={props.transform} />;
}

export function Face(props: { readonly entity: TopologicFaceEntity; readonly transform?: TopologicTransform }): ReactElement {
	const scene = useTopologicScene();
	const selectable = isEntitySelectable(scene, props.entity.id);
	const selected = useIsSelected(props.entity.id);
	const anchor = useMemo(() => centroid(props.entity.surface.vertices), [props.entity.surface.vertices]);
	const vertices = useMemo(() => props.entity.surface.vertices.map((point) => offsetPoint(point, anchor)), [anchor, props.entity.surface.vertices]);
	const geometry = useFaceGeometry(vertices, props.entity.surface.triangles);
	return (
		<TopologicGroup entityId={props.entity.id} transform={props.transform}>
			<mesh geometry={geometry} raycast={selectable ? undefined : disableRaycast}>
				<meshStandardMaterial
					color={selectedColor(topologyColor(props.entity, "#fbbf24"), selected)}
					transparent
					opacity={topologyOpacity(props.entity, 0.58)}
					side={DoubleSide}
				/>
			</mesh>
		</TopologicGroup>
	);
}

export function Shell(props: { readonly entity: TopologicShellEntity; readonly transform?: TopologicTransform }): ReactElement {
	return <TopologicContainer entityId={props.entity.id} transform={props.transform} />;
}

export function Cell(props: { readonly entity: TopologicCellEntity; readonly transform?: TopologicTransform }): ReactElement {
	return <TopologicContainer entityId={props.entity.id} transform={props.transform} />;
}

export function CellComplex(props: { readonly entity: TopologicCellComplexEntity; readonly transform?: TopologicTransform }): ReactElement {
	return <TopologicContainer entityId={props.entity.id} transform={props.transform} />;
}

export function Cluster(props: { readonly entity: TopologicClusterEntity; readonly transform?: TopologicTransform }): ReactElement {
	return <TopologicContainer entityId={props.entity.id} transform={props.transform} />;
}

function TopologyRoot(props: { readonly entity: TopologicTopologyEntity; readonly transform?: TopologicTransform }): ReactElement {
	return <TopologicContainer entityId={props.entity.id} transform={props.transform} />;
}

export function Topology(props: { readonly entry: ResolvedTopologyEntry }): ReactElement | null {
	const { entity, transform } = props.entry;
	if (entity.kind === "topology") return <TopologyRoot entity={entity} transform={transform} />;
	if (entity.kind === "vertex") return <Vertex entity={entity} transform={transform} />;
	if (entity.kind === "edge") return <Edge entity={entity} transform={transform} />;
	if (entity.kind === "wire") return <Wire entity={entity} transform={transform} />;
	if (entity.kind === "face") return <Face entity={entity} transform={transform} />;
	if (entity.kind === "shell") return <Shell entity={entity} transform={transform} />;
	if (entity.kind === "cell") return <Cell entity={entity} transform={transform} />;
	if (entity.kind === "cellComplex") return <CellComplex entity={entity} transform={transform} />;
	return <Cluster entity={entity} transform={transform} />;
}
//#endregion 🔖Kinds

//#region 🔖Transform
function isSceneDescendantOf(node: Object3D, ancestor: Object3D): boolean {
	let current: Object3D | null = node.parent;
	while (current) {
		if (current === ancestor) return true;
		current = current.parent;
	}
	return false;
}

function canAttachForDrag(object: Object3D, child: Object3D): boolean {
	if (object === child) return false;
	return !isSceneDescendantOf(object, child);
}

function TopologicTransformGumball(): ReactElement | null {
	const scene = useTopologicScene();
	const attachedRef = useRef<ReadonlyArray<{ readonly childId: string; readonly parent: Object3D }>>([]);
	const object = scene.selectedId ? scene.objectById.get(scene.selectedId) : null;
	const dragAttachIds = useMemo(
		() => (scene.selectedId ? collectDragAttachIds(scene.session, scene.selectedId) : []),
		[scene.selectedId, scene.session],
	);
	if (!scene.selectedId || !object) return null;
	return (
		<TransformControls
			object={object}
			mode={scene.transformMode}
			space="world"
			onMouseDown={() => {
				attachedRef.current = dragAttachIds.flatMap((linkedId) => {
					const child = scene.objectById.get(linkedId);
					if (!child?.parent || !canAttachForDrag(object, child)) return [];
					return [{ childId: linkedId, parent: child.parent }];
				});
				for (const { childId } of attachedRef.current) {
					const child = scene.objectById.get(childId);
					if (child) object.attach(child);
				}
			}}
			onMouseUp={() => {
				for (const { childId, parent } of attachedRef.current) {
					const child = scene.objectById.get(childId);
					if (child) parent.attach(child);
				}
				attachedRef.current = [];
				const position: Vec3 = [object.position.x, object.position.y, object.position.z];
				const rotation: TopologicTransform["rotation"] = [
					object.quaternion.x,
					object.quaternion.y,
					object.quaternion.z,
					object.quaternion.w,
				];
				const scale: TopologicTransform["scale"] = [object.scale.x, object.scale.y, object.scale.z];
				const commitTransform: TopologicTransform =
					scene.transformMode === "translate"
						? { position }
						: scene.transformMode === "rotate"
							? { position, rotation }
							: { position, rotation, scale };
				scene.onTransformCommit?.(scene.selectedId, commitTransform);
			}}
		/>
	);
}
//#endregion 🔖Transform

//#region 🔖Scene
export interface TopologicViewportProps {
	readonly fixture: TopologicFixtureV1;
	readonly selectedId?: string | null;
	readonly selectableKinds?: Readonly<Partial<Record<TopologicKind, boolean>>>;
	readonly onSelect?: (id: string | null) => void;
	readonly onTransformCommit?: (id: string, transform: TopologicTransform) => void;
	readonly transformMode?: TopologicTransformMode;
	readonly className?: string;
	readonly style?: CSSProperties;
	readonly backgroundColor?: string;
}

function TopologicSceneGraph(props: Omit<TopologicViewportProps, "className" | "style" | "backgroundColor">): ReactElement {
	const objectMapRef = useRef(new Map<string, Group>());
	const [version, setVersion] = useState(0);
	const session = useMemo(() => new TopologicWasmSession(props.fixture), [props.fixture]);
	const registerObject = useCallback((id: string, object: Group | null) => {
		const current = objectMapRef.current.get(id) ?? null;
		if (object === current) return;
		if (!object && !objectMapRef.current.has(id)) return;
		if (object) objectMapRef.current.set(id, object);
		else objectMapRef.current.delete(id);
		setVersion((current) => current + 1);
	}, []);
	const value = useMemo<TopologicSceneValue>(
		() => ({
			session,
			selectedId: props.selectedId ?? null,
			registerObject,
			objectById: objectMapRef.current,
			onSelect: props.onSelect,
			onTransformCommit: props.onTransformCommit,
			transformMode: props.transformMode ?? "translate",
			selectableKinds: props.selectableKinds,
		}),
		[props.onSelect, props.onTransformCommit, props.selectableKinds, props.selectedId, props.transformMode, registerObject, session, version],
	);
	const traversal = useMemo(() => collectSceneEntries(session), [session]);
	return (
		<TopologicSceneContext.Provider value={value}>
			<ambientLight intensity={0.55} />
			<directionalLight position={[9, 12, 8]} intensity={1} />
			<directionalLight position={[-8, 6, -6]} intensity={0.45} />
			<gridHelper args={[32, 32, "#334155", "#1e293b"]} />
			<axesHelper args={[3.5]} />
			{traversal.entries.map((entry) => (
				<Topology key={entry.entity.id} entry={entry} />
			))}
			<TopologicTransformGumball />
			<OrbitControls makeDefault />
		</TopologicSceneContext.Provider>
	);
}

export function TopologicViewport(props: TopologicViewportProps): ReactElement {
	return (
		<div className={props.className} style={{ width: "100%", height: "100%", ...(props.style ?? {}) }}>
			<Canvas camera={{ position: [8, 6, 8], near: 0.1, far: 1000, fov: 45 }}>
				<color attach="background" args={[props.backgroundColor ?? "#09111f"]} />
				<TopologicSceneGraph {...props} />
			</Canvas>
		</div>
	);
}
//#endregion 🔖Scene

//#region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("TopologicViewport helpers", () => {
		it("keeps the published kind order stable for UI selection", () => {
			expect(TOPOLOGIC_KINDS).toEqual([
				"topology",
				"vertex",
				"edge",
				"wire",
				"face",
				"shell",
				"cell",
				"cellComplex",
				"cluster",
			]);
		});

		it("parses the shipped fixture through the wasm facade contract", async () => {
			const bindings = await ensureTopologicWasmLoaded();
			const fixture = bindings.parseFixture((await import("../fixtures/topology.json")).default);
			expect(fixture?.schema).toBe("elements.geometry.topologic.fixture/v1");
		});

		it("collects every shipped topology from the rooted scene graph", async () => {
			const bindings = await ensureTopologicWasmLoaded();
			const fixture = bindings.parseFixture((await import("../fixtures/topology.json")).default);
			expect(fixture).toBeTruthy();
			const traversal = collectSceneEntries(new TopologicWasmSession(fixture as TopologicFixtureV1));
			expect(traversal.entries).toHaveLength((fixture as TopologicFixtureV1).topologies.length);
		});

		it("keeps edge group transforms world-aligned for translate gumball axes", async () => {
			const fixture = (await loadTopologicFixtureV1({
				schema: "elements.geometry.topologic.fixture/v1",
				roots: ["root"],
				topologies: [
					{ id: "root", kind: "topology", members: ["edge"] },
					{ id: "start", kind: "vertex", point: [0, 0, 0] },
					{ id: "end", kind: "vertex", point: [3, 4, 0] },
					{
						id: "edge",
						kind: "edge",
						vertices: ["start", "end"],
						transform: { rotation: [0, 0, 0.3826834323650898, 0.9238795325112867] },
					},
				],
			})) as TopologicFixtureV1;
			const session = new TopologicWasmSession(fixture);
			const edge = session.getEntity("edge") as TopologicEdgeEntity;
			const group = resolveEntityGroupTransform(session, edge, edge.transform);
			expect(group?.rotation).toEqual([0, 0, 0, 1]);
		});

		it("builds pick segments for straight and curved edges", () => {
			expect(edgePickSegments([[0, 0, 0], [2, 0, 0]])).toEqual([[[0, 0, 0], [2, 0, 0]]]);
			expect(edgePickSegments([[0, 0, 0], [1, 0, 0], [1, 1, 0]])).toEqual([
				[[0, 0, 0], [1, 0, 0]],
				[[1, 0, 0], [1, 1, 0]],
			]);
		});
	});
}
//#endregion 🧪Tests
