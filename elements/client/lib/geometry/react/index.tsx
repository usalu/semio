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
import { BufferGeometry, DoubleSide, Float32BufferAttribute, Group } from "three";

import {
	TOPOLOGIC_KINDS,
	TopologicCellComplexEntity,
	TopologicCellEntity,
	TopologicClusterEntity,
	TopologicEdgeEntity,
	TopologicFaceEntity,
	type TopologicFixtureV1,
	TopologicShellEntity,
	TopologicTopologyEntity,
	type TopologicTransform,
	TopologicVertexEntity,
	TopologicWasmSession,
	TopologicWireEntity,
	ensureTopologicWasmLoaded,
	normalizeTransform,
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
	const { position, quaternion, scale } = transformProps(props.transform);
	useEffect(() => {
		scene.registerObject(props.entityId, ref.current);
		return () => scene.registerObject(props.entityId, null);
	}, [props.entityId, scene]);
	return (
		<group
			ref={ref}
			position={position}
			quaternion={quaternion}
			scale={scale}
			onPointerDown={(event) => {
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
function useFaceGeometry(face: TopologicFaceEntity): BufferGeometry {
	const geometry = useMemo(() => {
		const next = new BufferGeometry();
		next.setAttribute("position", new Float32BufferAttribute(face.surface.vertices.flat(), 3));
		next.setIndex([...face.surface.triangles]);
		next.computeVertexNormals();
		return next;
	}, [face.surface.triangles, face.surface.vertices]);
	useEffect(() => () => geometry.dispose(), [geometry]);
	return geometry;
}
//#endregion 🔖Geometry

//#region 🔖Kinds
export function Vertex(props: { readonly entity: TopologicVertexEntity }): ReactElement {
	const selected = useIsSelected(props.entity.id);
	const color = selectedColor(topologyColor(props.entity, "#38bdf8"), selected);
	const radius = props.entity.radius ?? props.entity.style?.pointSize ?? 0.12;
	return (
		<TopologicGroup entityId={props.entity.id} transform={props.entity.transform}>
			<mesh position={props.entity.point}>
				<sphereGeometry args={[radius, 24, 24]} />
				<meshStandardMaterial color={color} transparent opacity={topologyOpacity(props.entity, 1)} />
			</mesh>
		</TopologicGroup>
	);
}

export function Edge(props: { readonly entity: TopologicEdgeEntity }): ReactElement {
	const scene = useTopologicScene();
	const selected = useIsSelected(props.entity.id);
	const points = scene.session.edgeCurve(props.entity.id);
	return (
		<TopologicGroup entityId={props.entity.id} transform={props.entity.transform}>
			<Line
				points={points}
				color={selectedColor(topologyColor(props.entity, "#f8fafc"), selected)}
				lineWidth={topologyLineWidth(props.entity, 2)}
			/>
		</TopologicGroup>
	);
}

export function Wire(props: { readonly entity: TopologicWireEntity }): ReactElement {
	const scene = useTopologicScene();
	const selected = useIsSelected(props.entity.id);
	return (
		<TopologicGroup entityId={props.entity.id} transform={props.entity.transform}>
			{props.entity.edges.map((edgeId) => {
				const edge = scene.session.getEntity(edgeId);
				if (!edge || edge.kind !== "edge") return null;
				return (
					<Line
						key={edgeId}
						points={scene.session.edgeCurve(edgeId)}
						color={selectedColor(topologyColor(props.entity, edge.style?.color ?? "#34d399"), selected)}
						lineWidth={topologyLineWidth(props.entity, edge.style?.lineWidth ?? 1.5)}
					/>
				);
			})}
		</TopologicGroup>
	);
}

export function Face(props: { readonly entity: TopologicFaceEntity }): ReactElement {
	const selected = useIsSelected(props.entity.id);
	const geometry = useFaceGeometry(props.entity);
	return (
		<TopologicGroup entityId={props.entity.id} transform={props.entity.transform}>
			<mesh geometry={geometry}>
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

export function Shell(props: { readonly entity: TopologicShellEntity }): ReactElement {
	const scene = useTopologicScene();
	return (
		<TopologicGroup entityId={props.entity.id} transform={props.entity.transform}>
			{props.entity.faces.map((faceId) => {
				const face = scene.session.getEntity(faceId);
				return face?.kind === "face" ? <Face key={faceId} entity={face} /> : null;
			})}
		</TopologicGroup>
	);
}

export function Cell(props: { readonly entity: TopologicCellEntity }): ReactElement {
	const scene = useTopologicScene();
	return (
		<TopologicGroup entityId={props.entity.id} transform={props.entity.transform}>
			{props.entity.shells.map((shellId) => {
				const shell = scene.session.getEntity(shellId);
				return shell?.kind === "shell" ? <Shell key={shellId} entity={shell} /> : null;
			})}
		</TopologicGroup>
	);
}

export function CellComplex(props: { readonly entity: TopologicCellComplexEntity }): ReactElement {
	const scene = useTopologicScene();
	return (
		<TopologicGroup entityId={props.entity.id} transform={props.entity.transform}>
			{props.entity.cells.map((cellId) => {
				const cell = scene.session.getEntity(cellId);
				return cell?.kind === "cell" ? <Cell key={cellId} entity={cell} /> : null;
			})}
		</TopologicGroup>
	);
}

export function Cluster(props: { readonly entity: TopologicClusterEntity }): ReactElement {
	return (
		<TopologicGroup entityId={props.entity.id} transform={props.entity.transform}>
			{props.entity.topologies.map((topologyId) => (
				<Topology key={topologyId} entityId={topologyId} />
			))}
		</TopologicGroup>
	);
}

function TopologyRoot(props: { readonly entity: TopologicTopologyEntity }): ReactElement {
	return (
		<TopologicGroup entityId={props.entity.id} transform={props.entity.transform}>
			{props.entity.members.map((memberId) => (
				<Topology key={memberId} entityId={memberId} />
			))}
		</TopologicGroup>
	);
}

export function Topology(props: { readonly entityId: string }): ReactElement | null {
	const scene = useTopologicScene();
	const entity = scene.session.getEntity(props.entityId);
	if (!entity) return null;
	if (entity.kind === "topology") return <TopologyRoot entity={entity} />;
	if (entity.kind === "vertex") return <Vertex entity={entity} />;
	if (entity.kind === "edge") return <Edge entity={entity} />;
	if (entity.kind === "wire") return <Wire entity={entity} />;
	if (entity.kind === "face") return <Face entity={entity} />;
	if (entity.kind === "shell") return <Shell entity={entity} />;
	if (entity.kind === "cell") return <Cell entity={entity} />;
	if (entity.kind === "cellComplex") return <CellComplex entity={entity} />;
	return <Cluster entity={entity} />;
}
//#endregion 🔖Kinds

//#region 🔖Transform
function TopologicTransformGumball(): ReactElement | null {
	const scene = useTopologicScene();
	const object = scene.selectedId ? scene.objectById.get(scene.selectedId) : null;
	if (!scene.selectedId || !object) return null;
	return (
		<TransformControls
			object={object}
			mode={scene.transformMode}
			onMouseUp={() => {
				scene.onTransformCommit?.(scene.selectedId, {
					position: [object.position.x, object.position.y, object.position.z],
					rotation: [object.quaternion.x, object.quaternion.y, object.quaternion.z, object.quaternion.w],
					scale: [object.scale.x, object.scale.y, object.scale.z],
				});
			}}
		/>
	);
}
//#endregion 🔖Transform

//#region 🔖Scene
export interface TopologicViewportProps {
	readonly fixture: TopologicFixtureV1;
	readonly selectedId?: string | null;
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
		}),
		[props.onSelect, props.onTransformCommit, props.selectedId, props.transformMode, registerObject, session, version],
	);
	return (
		<TopologicSceneContext.Provider value={value}>
			<ambientLight intensity={0.55} />
			<directionalLight position={[9, 12, 8]} intensity={1} />
			<directionalLight position={[-8, 6, -6]} intensity={0.45} />
			<gridHelper args={[32, 32, "#334155", "#1e293b"]} />
			<axesHelper args={[3.5]} />
			{session.fixture.roots.map((entityId) => (
				<Topology key={entityId} entityId={entityId} />
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
	});
}
//#endregion 🧪Tests
