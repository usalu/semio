import { useCallback, useEffect, useMemo, useRef, useState, Suspense, type ComponentProps } from "react";
import {
	BufferAttribute,
	BufferGeometry,
	DoubleSide,
	Group,
	LineBasicMaterial,
	MeshBasicMaterial,
	MeshStandardMaterial,
	Object3D,
	PointsMaterial,
	Quaternion,
	TextureLoader,
	Vector3,
	type ThreeEvent,
} from "three";
import { useLoader, useThree } from "@react-three/fiber";
import { GLTFLoader } from "three/examples/jsm/loaders/GLTFLoader.js";
import {
	DEFAULT_LOD_GRID_FACTOR,
	DEFAULT_MANUAL_LOD,
	WorldCanvas,
	WorldLayerStack,
	WorldLodBridge,
	WorldOrbitCameraViewRig,
	WorldOrbitGated,
	WorldOrbitViewSnapGateProvider,
	type WorldCameraState,
} from "@semio-tech/infinite-world-r3f";
import {
	UnifiedGumball,
	marqueeCoverageFromGesture,
	marqueeModeFromModifiers,
	SelectionMarquee,
	type GumballConfig,
	type GumballHandleKind,
	type GumballPose,
	type SelectionMarqueeMethod,
	type SelectionMarqueePoint,
} from "@semio-tech/ui-react";
import { resolveSemanticColorHex } from "@semio-tech/ui-styling";
import type { CommandDescriptor, UiComponentSceneNode } from "../os-shell.tsx";

//#region WorldSceneParsing
type WorldMeshData = {
	readonly positions: readonly number[];
	readonly normals: readonly number[];
	readonly indices: readonly number[];
	readonly uvs?: readonly number[];
	readonly faceIds?: readonly number[];
	readonly vertexIds?: readonly number[];
	readonly edgePositions?: readonly number[];
	readonly edgeIds?: readonly number[];
	readonly paintTextureBase64?: string;
};

type WorldCameraRecord = {
	readonly position?: readonly [number, number, number];
	readonly target?: readonly [number, number, number];
	readonly fov?: number;
	readonly x?: number;
	readonly y?: number;
	readonly z?: number;
};

type WorldMeshRecord = {
	readonly id: string;
	readonly data?: WorldMeshData;
	readonly url?: string;
};

type WorldInstanceRecord = {
	readonly id: string;
	readonly meshId?: string;
	readonly position?: readonly [number, number, number];
	readonly rotation?: readonly [number, number, number, number];
	readonly scale?: readonly [number, number, number];
	readonly x?: number;
	readonly y?: number;
	readonly z?: number;
	readonly selected?: boolean;
	readonly hovered?: boolean;
	readonly smoothShading?: boolean;
};

type WorldSelectionTargets = {
	readonly mesh?: boolean;
	readonly vertex?: boolean;
	readonly edge?: boolean;
	readonly face?: boolean;
};

type WorldHoverComponent = {
	readonly objectId?: string;
	readonly mode?: string;
	readonly id?: number;
};

type WorldSelectionRecord = {
	readonly method?: SelectionMarqueeMethod;
	readonly ids?: readonly string[];
	readonly granularity?: string;
	readonly selectionMode?: string;
	readonly activeObjectId?: string;
	readonly componentIds?: readonly number[];
	readonly targets?: WorldSelectionTargets;
	readonly transformTool?: string;
	readonly interactionMode?: "model" | "paint";
	readonly gumballTarget?: readonly [number, number, number];
	readonly gumballActive?: boolean;
	readonly hoveredComponent?: WorldHoverComponent;
	readonly showEdges?: boolean;
	readonly engagementSessionActive?: boolean;
};

type SemanticColors = {
	readonly mesh: string;
	readonly edge: string;
	readonly select: string;
	readonly hover: string;
};

function useSemanticColors(): SemanticColors {
	const resolve = useCallback(
		(): SemanticColors => ({
			mesh: resolveSemanticColorHex("--panel"),
			edge: resolveSemanticColorHex("--border-normal-color"),
			select: resolveSemanticColorHex("--active-base"),
			hover: resolveSemanticColorHex("--hover-base"),
		}),
		[],
	);
	const [colors, setColors] = useState(resolve);
	useEffect(() => {
		const observer = new MutationObserver(() => setColors(resolve()));
		observer.observe(document.documentElement, { attributes: true, attributeFilter: ["data-theme", "class"] });
		return () => observer.disconnect();
	}, [resolve]);
	return colors;
}

function parseCameraState(cameraJson: string): WorldCameraState & { readonly fov: number } {
	try {
		const parsed = JSON.parse(cameraJson) as WorldCameraRecord & { target?: readonly [number, number, number] };
		const position: [number, number, number] = parsed.position
			? [parsed.position[0], parsed.position[1], parsed.position[2]]
			: [parsed.x ?? 4, parsed.y ?? -4, parsed.z ?? 3];
		const target: [number, number, number] = parsed.target
			? [parsed.target[0], parsed.target[1], parsed.target[2]]
			: [0, 0, 0];
		return { position, target, zoom: 1, projection: "perspective", fov: parsed.fov ?? 45 };
	} catch {
		return { position: [4, -4, 3], target: [0, 0, 0], zoom: 1, projection: "perspective", fov: 45 };
	}
}

function parseMeshes(meshesJson: string): WorldMeshRecord[] {
	try {
		const parsed = JSON.parse(meshesJson);
		return Array.isArray(parsed) ? (parsed as WorldMeshRecord[]) : [];
	} catch {
		return [];
	}
}

function parseInstances(instancesJson: string): WorldInstanceRecord[] {
	try {
		const parsed = JSON.parse(instancesJson);
		return Array.isArray(parsed) ? (parsed as WorldInstanceRecord[]) : [];
	} catch {
		return [];
	}
}

function parseSelection(selectionJson: string): WorldSelectionRecord {
	try {
		return JSON.parse(selectionJson) as WorldSelectionRecord;
	} catch {
		return { method: "rectangle", ids: [] };
	}
}

function geometryFromMesh(mesh: WorldMeshData) {
	const geometry = new BufferGeometry();
	geometry.setAttribute("position", new BufferAttribute(new Float32Array(mesh.positions), 3));
	geometry.setAttribute("normal", new BufferAttribute(new Float32Array(mesh.normals), 3));
	if (mesh.uvs?.length) geometry.setAttribute("uv", new BufferAttribute(new Float32Array(mesh.uvs), 2));
	if (mesh.indices.length > 0) geometry.setIndex([...mesh.indices]);
	return geometry;
}

type VertexPickData = {
	readonly geometry: BufferGeometry;
	readonly vertexIds: readonly number[];
};

function buildVertexPickData(mesh: WorldMeshData): VertexPickData | null {
	if (!mesh.vertexIds?.length) return null;
	const positions: number[] = [];
	const vertexIds: number[] = [];
	const emitted = new Set<number>();
	for (let index = 0; index < mesh.vertexIds.length; index += 1) {
		const id = mesh.vertexIds[index]!;
		if (emitted.has(id)) continue;
		emitted.add(id);
		vertexIds.push(id);
		positions.push(mesh.positions[index * 3]!, mesh.positions[index * 3 + 1]!, mesh.positions[index * 3 + 2]!);
	}
	if (!positions.length) return null;
	const geometry = new BufferGeometry();
	geometry.setAttribute("position", new BufferAttribute(new Float32Array(positions), 3));
	return { geometry, vertexIds };
}

function buildEdgeGeometry(mesh: WorldMeshData): BufferGeometry | null {
	if (!mesh.edgePositions?.length) return null;
	const geometry = new BufferGeometry();
	geometry.setAttribute("position", new BufferAttribute(new Float32Array(mesh.edgePositions), 3));
	return geometry;
}

function buildFaceOverlayGeometry(mesh: WorldMeshData, faceIds: ReadonlySet<number>): BufferGeometry | null {
	if (!mesh.faceIds?.length || !mesh.indices.length || faceIds.size === 0) return null;
	const positions: number[] = [];
	const normals: number[] = [];
	for (let faceIndex = 0; faceIndex < mesh.faceIds.length; faceIndex += 1) {
		const faceId = mesh.faceIds[faceIndex]!;
		if (!faceIds.has(faceId)) continue;
		const i0 = mesh.indices[faceIndex * 3] ?? 0;
		const i1 = mesh.indices[faceIndex * 3 + 1] ?? 0;
		const i2 = mesh.indices[faceIndex * 3 + 2] ?? 0;
		for (const index of [i0, i1, i2]) {
			positions.push(mesh.positions[index * 3]!, mesh.positions[index * 3 + 1]!, mesh.positions[index * 3 + 2]!);
			normals.push(mesh.normals[index * 3]!, mesh.normals[index * 3 + 1]!, mesh.normals[index * 3 + 2]!);
		}
	}
	if (!positions.length) return null;
	const geometry = new BufferGeometry();
	geometry.setAttribute("position", new BufferAttribute(new Float32Array(positions), 3));
	geometry.setAttribute("normal", new BufferAttribute(new Float32Array(normals), 3));
	return geometry;
}

function buildEdgeOverlayGeometry(mesh: WorldMeshData, edgeIds: ReadonlySet<number>): BufferGeometry | null {
	if (!mesh.edgeIds?.length || !mesh.edgePositions?.length || edgeIds.size === 0) return null;
	const positions: number[] = [];
	for (let edgeIndex = 0; edgeIndex < mesh.edgeIds.length; edgeIndex += 1) {
		if (!edgeIds.has(mesh.edgeIds[edgeIndex]!)) continue;
		const base = edgeIndex * 6;
		positions.push(
			mesh.edgePositions[base]!,
			mesh.edgePositions[base + 1]!,
			mesh.edgePositions[base + 2]!,
			mesh.edgePositions[base + 3]!,
			mesh.edgePositions[base + 4]!,
			mesh.edgePositions[base + 5]!,
		);
	}
	if (!positions.length) return null;
	const geometry = new BufferGeometry();
	geometry.setAttribute("position", new BufferAttribute(new Float32Array(positions), 3));
	return geometry;
}

function buildVertexOverlayGeometry(mesh: WorldMeshData, vertexIds: ReadonlySet<number>): BufferGeometry | null {
	const pick = buildVertexPickData(mesh);
	if (!pick) return null;
	const positions: number[] = [];
	for (let index = 0; index < pick.vertexIds.length; index += 1) {
		if (!vertexIds.has(pick.vertexIds[index]!)) continue;
		positions.push(
			pick.geometry.attributes.position!.getX(index),
			pick.geometry.attributes.position!.getY(index),
			pick.geometry.attributes.position!.getZ(index),
		);
	}
	if (!positions.length) return null;
	const geometry = new BufferGeometry();
	geometry.setAttribute("position", new BufferAttribute(new Float32Array(positions), 3));
	return geometry;
}

function paintTextureUrl(base64: string): string {
	return `data:image/png;base64,${base64}`;
}

function PaintTexturedMesh({
	geometry,
	color,
	textureBase64,
	flatShading,
	children,
	...meshProps
}: {
	readonly geometry: BufferGeometry;
	readonly color: string;
	readonly textureBase64?: string;
	readonly flatShading?: boolean;
	readonly children?: React.ReactNode;
} & ComponentProps<"mesh">) {
	const paintMap = textureBase64 ? useLoader(TextureLoader, paintTextureUrl(textureBase64)) : null;
	return (
		<mesh geometry={geometry} {...meshProps}>
			<meshStandardMaterial color={color} map={paintMap ?? undefined} side={DoubleSide} flatShading={flatShading} />
			{children}
		</mesh>
	);
}

function GlbInstanceMesh({ url, color }: { readonly url: string; readonly color: string }) {
	const gltf = useLoader(GLTFLoader, url);
	const scene = useMemo(() => {
		const cloned = gltf.scene.clone(true);
		cloned.traverse((child) => {
			if ("isMesh" in child && (child as { isMesh?: boolean }).isMesh) {
				const mesh = child as import("three").Mesh;
				mesh.material = new MeshStandardMaterial({ color });
			}
		});
		return cloned;
	}, [gltf.scene, color]);
	return <primitive object={scene} />;
}

function gumballConfigForTransformTool(tool: string): GumballConfig {
	if (tool === "rotate") {
		return { moveAxes: false, movePlanes: false, rotate: true, scaleAxes: false, scalePlanes: false, scaleUniform: false };
	}
	if (tool === "scale") {
		return { moveAxes: false, movePlanes: false, rotate: false, scaleAxes: true, scalePlanes: true, scaleUniform: true };
	}
	return { moveAxes: true, movePlanes: true, rotate: false, scaleAxes: false, scalePlanes: false, scaleUniform: false };
}

function SceneGumball({
	target,
	config,
	active,
	onDraggingChanged,
	onDragEnd,
}: {
	readonly target?: readonly [number, number, number];
	readonly config: GumballConfig;
	readonly active: boolean;
	readonly onDraggingChanged: (dragging: boolean) => void;
	readonly onDragEnd: (kind: GumballHandleKind, before: GumballPose, after: GumballPose) => void;
}) {
	const pivotRef = useRef<Object3D>(new Object3D());
	const [ready, setReady] = useState(false);
	useEffect(() => {
		if (!target) return;
		pivotRef.current.position.set(target[0], target[1], target[2]);
		pivotRef.current.quaternion.set(0, 0, 0, 1);
		pivotRef.current.scale.set(1, 1, 1);
		pivotRef.current.updateMatrixWorld(true);
		setReady(true);
	}, [target]);
	if (!active || !target || !ready) return null;
	return (
		<>
			<primitive object={pivotRef.current} />
			<UnifiedGumball
				target={pivotRef.current}
				config={config}
				onDraggingChanged={onDraggingChanged}
				onDragEnd={(kind, before, after) => {
					onDragEnd(kind, before, after);
					pivotRef.current.position.set(target[0], target[1], target[2]);
					pivotRef.current.quaternion.set(0, 0, 0, 1);
					pivotRef.current.scale.set(1, 1, 1);
					pivotRef.current.updateMatrixWorld(true);
				}}
			/>
		</>
	);
}

function WorldInstanceNode({
	instance,
	index,
	meshRecord,
	meshData,
	geometry,
	colors,
	vertexPick,
	edgeGeometry,
	paintTextureBase64,
	position,
	scale,
	quaternion,
	targets,
	activeObjectId,
	selectionMode,
	selectedComponentIds,
	previewComponentIds,
	hoveredComponent,
	showEdges,
	pickEnabled,
	onPaintAt,
	paintFromHit,
	flatShading,
	onInstancePointerDown,
	onInstancePointerMove,
	onWorldPick,
	onComponentHover,
	mergeMode,
}: {
	readonly instance: WorldInstanceRecord;
	readonly index: number;
	readonly meshRecord?: WorldMeshRecord;
	readonly meshData?: WorldMeshData;
	readonly geometry?: BufferGeometry;
	readonly colors: SemanticColors;
	readonly vertexPick: VertexPickData | null;
	readonly edgeGeometry: BufferGeometry | null;
	readonly paintTextureBase64?: string;
	readonly position: readonly [number, number, number];
	readonly scale: readonly [number, number, number];
	readonly quaternion?: Quaternion;
	readonly targets: WorldSelectionTargets;
	readonly activeObjectId?: string;
	readonly selectionMode: string;
	readonly selectedComponentIds: ReadonlySet<number>;
	readonly previewComponentIds: ReadonlySet<number>;
	readonly hoveredComponent?: WorldHoverComponent;
	readonly showEdges?: boolean;
	readonly pickEnabled: boolean;
	readonly onPaintAt?: (objectId: string, u: number, v: number) => void;
	readonly paintFromHit: (
		objectId: string,
		mesh: WorldMeshData,
		event: ThreeEvent<PointerEvent> & { faceIndex?: number | null; uv?: { x: number; y: number } },
	) => void;
	readonly flatShading?: boolean;
	readonly onInstancePointerDown: (id: string, index: number, event: { shiftKey: boolean; ctrlKey: boolean; metaKey: boolean }) => void;
	readonly onInstancePointerMove: (id: string | null) => void;
	readonly onWorldPick: (args: { granularity: string; id: number; merge: string }) => void;
	readonly onComponentHover: (args: { objectId: string; mode: string; id: number } | null) => void;
	readonly mergeMode: (event: { shiftKey?: boolean; ctrlKey?: boolean; metaKey?: boolean }) => string;
}) {
	const isActiveObject = instance.id === activeObjectId;
	const meshTint = instance.selected ? colors.select : instance.hovered ? colors.hover : colors.mesh;
	const hoveredFaceId =
		isActiveObject && hoveredComponent?.mode === "face" && hoveredComponent.objectId === instance.id
			? hoveredComponent.id
			: undefined;
	const hoveredVertexId =
		isActiveObject && hoveredComponent?.mode === "vertex" && hoveredComponent.objectId === instance.id
			? hoveredComponent.id
			: undefined;
	const hoveredEdgeId =
		isActiveObject && hoveredComponent?.mode === "edge" && hoveredComponent.objectId === instance.id
			? hoveredComponent.id
			: undefined;
	const selectedFaceIds = isActiveObject && selectionMode === "face" ? selectedComponentIds : new Set<number>();
	const selectedVertexIds = isActiveObject && selectionMode === "vertex" ? selectedComponentIds : new Set<number>();
	const selectedEdgeIds = isActiveObject && selectionMode === "edge" ? selectedComponentIds : new Set<number>();
	const previewFaceIds = isActiveObject && selectionMode === "face" ? previewComponentIds : new Set<number>();
	const previewVertexIds = isActiveObject && selectionMode === "vertex" ? previewComponentIds : new Set<number>();
	const previewEdgeIds = isActiveObject && selectionMode === "edge" ? previewComponentIds : new Set<number>();
	const facePreviewOverlay =
		meshData && previewFaceIds.size > 0 ? buildFaceOverlayGeometry(meshData, previewFaceIds) : null;
	const edgePreviewOverlay =
		meshData && previewEdgeIds.size > 0 ? buildEdgeOverlayGeometry(meshData, previewEdgeIds) : null;
	const vertexPreviewOverlay =
		meshData && previewVertexIds.size > 0 ? buildVertexOverlayGeometry(meshData, previewVertexIds) : null;
	const faceSelectedOverlay = meshData ? buildFaceOverlayGeometry(meshData, selectedFaceIds) : null;
	const faceHoveredOverlay =
		meshData && hoveredFaceId != null ? buildFaceOverlayGeometry(meshData, new Set([hoveredFaceId])) : null;
	const edgeSelectedOverlay = meshData ? buildEdgeOverlayGeometry(meshData, selectedEdgeIds) : null;
	const edgeHoveredOverlay =
		meshData && hoveredEdgeId != null ? buildEdgeOverlayGeometry(meshData, new Set([hoveredEdgeId])) : null;
	const vertexSelectedOverlay = meshData ? buildVertexOverlayGeometry(meshData, selectedVertexIds) : null;
	const vertexHoveredOverlay =
		meshData && hoveredVertexId != null ? buildVertexOverlayGeometry(meshData, new Set([hoveredVertexId])) : null;

	return (
		<group position={position as [number, number, number]} scale={scale as [number, number, number]} quaternion={quaternion}>
			{geometry && meshData ? (
				<>
					<PaintTexturedMesh
						geometry={geometry}
						color={meshTint}
						textureBase64={paintTextureBase64}
						flatShading={flatShading}
						onClick={(event) => {
							if (onPaintAt) {
								paintFromHit(instance.id, meshData, event);
								return;
							}
							if (!pickEnabled) return;
							event.stopPropagation();
							if (targets.face && event.faceIndex != null && meshData.faceIds?.[event.faceIndex] != null) {
								onWorldPick({
									granularity: "face",
									id: meshData.faceIds[event.faceIndex]!,
									merge: mergeMode(event),
								});
							} else if (targets.mesh) {
								onInstancePointerDown(instance.id, index, event);
							}
						}}
						onPointerMove={(event) => {
							if (onPaintAt) {
								if ((event.buttons & 1) !== 0) paintFromHit(instance.id, meshData, event);
								return;
							}
							if (!pickEnabled) return;
							event.stopPropagation();
							if (targets.face && event.faceIndex != null && meshData.faceIds?.[event.faceIndex] != null) {
								onComponentHover({
									objectId: instance.id,
									mode: "face",
									id: meshData.faceIds[event.faceIndex]!,
								});
							} else {
								onInstancePointerMove(instance.id);
							}
						}}
						onPointerOut={() => {
							onInstancePointerMove(null);
							onComponentHover(null);
						}}
					/>
					{(targets.edge
						|| showEdges
						|| (selectionMode === "mesh" && selectedComponentIds.size > 0)) && edgeGeometry ? (
						<lineSegments
							geometry={edgeGeometry}
							onClick={(event) => {
								if (!pickEnabled || !meshData?.edgeIds?.length) return;
								event.stopPropagation();
								const edgeIndex = event.index ?? 0;
								const edgeId = meshData.edgeIds[edgeIndex];
								if (edgeId == null) return;
								onWorldPick({ granularity: "edge", id: edgeId, merge: mergeMode(event) });
							}}
							onPointerMove={(event) => {
								if (!pickEnabled || !meshData?.edgeIds?.length) return;
								event.stopPropagation();
								const edgeIndex = event.index ?? 0;
								const edgeId = meshData.edgeIds[edgeIndex];
								if (edgeId == null) return;
								onComponentHover({ objectId: instance.id, mode: "edge", id: edgeId });
							}}
							onPointerOut={() => onComponentHover(null)}
						>
							<lineBasicMaterial color={colors.edge} linewidth={1} />
						</lineSegments>
					) : null}
					{targets.vertex && vertexPick ? (
						<points
							geometry={vertexPick.geometry}
							onClick={(event) => {
								if (!pickEnabled) return;
								event.stopPropagation();
								const idx = event.index ?? 0;
								const vertexId = vertexPick.vertexIds[idx];
								if (vertexId == null) return;
								onWorldPick({ granularity: "vertex", id: vertexId, merge: mergeMode(event) });
							}}
							onPointerMove={(event) => {
								if (!pickEnabled) return;
								event.stopPropagation();
								const idx = event.index ?? 0;
								const vertexId = vertexPick.vertexIds[idx];
								if (vertexId == null) return;
								onComponentHover({ objectId: instance.id, mode: "vertex", id: vertexId });
							}}
							onPointerOut={() => onComponentHover(null)}
						>
							<pointsMaterial color={colors.edge} size={0.05} sizeAttenuation />
						</points>
					) : null}
					{faceSelectedOverlay ? (
						<mesh geometry={faceSelectedOverlay} raycast={() => null}>
							<meshBasicMaterial
								color={colors.select}
								transparent
								opacity={0.62}
								side={DoubleSide}
								depthWrite={false}
								polygonOffset
								polygonOffsetFactor={-2}
							/>
						</mesh>
					) : null}
					{faceHoveredOverlay ? (
						<mesh geometry={faceHoveredOverlay} raycast={() => null}>
							<meshBasicMaterial
								color={colors.hover}
								transparent
								opacity={0.48}
								side={DoubleSide}
								depthWrite={false}
								polygonOffset
								polygonOffsetFactor={-3}
							/>
						</mesh>
					) : null}
					{facePreviewOverlay ? (
						<mesh geometry={facePreviewOverlay} raycast={() => null}>
							<meshBasicMaterial
								color={colors.hover}
								transparent
								opacity={0.36}
								side={DoubleSide}
								depthWrite={false}
								polygonOffset
								polygonOffsetFactor={-4}
							/>
						</mesh>
					) : null}
					{edgeSelectedOverlay ? (
						<lineSegments geometry={edgeSelectedOverlay} raycast={() => null}>
							<lineBasicMaterial color={colors.select} linewidth={3} />
						</lineSegments>
					) : null}
					{edgeHoveredOverlay ? (
						<lineSegments geometry={edgeHoveredOverlay} raycast={() => null}>
							<lineBasicMaterial color={colors.hover} linewidth={3} />
						</lineSegments>
					) : null}
					{edgePreviewOverlay ? (
						<lineSegments geometry={edgePreviewOverlay} raycast={() => null}>
							<lineBasicMaterial color={colors.hover} linewidth={2} />
						</lineSegments>
					) : null}
					{vertexSelectedOverlay ? (
						<points geometry={vertexSelectedOverlay} raycast={() => null}>
							<pointsMaterial color={colors.select} size={0.09} sizeAttenuation depthTest={false} />
						</points>
					) : null}
					{vertexHoveredOverlay ? (
						<points geometry={vertexHoveredOverlay} raycast={() => null}>
							<pointsMaterial color={colors.hover} size={0.09} sizeAttenuation depthTest={false} />
						</points>
					) : null}
					{vertexPreviewOverlay ? (
						<points geometry={vertexPreviewOverlay} raycast={() => null}>
							<pointsMaterial color={colors.hover} size={0.09} sizeAttenuation depthTest={false} />
						</points>
					) : null}
				</>
			) : meshRecord?.url ? (
				<group
					onPointerDown={(event) => {
						event.stopPropagation();
						onInstancePointerDown(instance.id, index, event);
					}}
					onPointerMove={(event) => {
						event.stopPropagation();
						onInstancePointerMove(instance.id);
					}}
					onPointerOut={() => onInstancePointerMove(null)}
				>
					<Suspense fallback={null}>
						<GlbInstanceMesh url={meshRecord.url} color={meshTint} />
					</Suspense>
				</group>
			) : (
				<mesh
					onPointerDown={(event) => {
						event.stopPropagation();
						onInstancePointerDown(instance.id, index, event);
					}}
				>
					<boxGeometry args={[1, 1, 1]} />
					<meshStandardMaterial color={meshTint} />
				</mesh>
			)}
		</group>
	);
}
//#endregion WorldSceneParsing

//#region WorldInstancesLayer
function WorldInstancesLayer({
	instances,
	meshes,
	selection,
	colors,
	onInstancePointerDown,
	onInstancePointerMove,
	onWorldPick,
	onComponentHover,
	onPaintAt,
	gumballDragActive,
	onGumballDraggingChanged,
	onGumballDragEnd,
	previewComponentIds,
}: {
	readonly instances: readonly WorldInstanceRecord[];
	readonly meshes: readonly WorldMeshRecord[];
	readonly selection: WorldSelectionRecord;
	readonly colors: SemanticColors;
	readonly onInstancePointerDown: (id: string, index: number, event: { shiftKey: boolean; ctrlKey: boolean; metaKey: boolean }) => void;
	readonly onInstancePointerMove: (id: string | null) => void;
	readonly onWorldPick: (args: { granularity: string; id: number; merge: string }) => void;
	readonly onComponentHover: (args: { objectId: string; mode: string; id: number } | null) => void;
	readonly onPaintAt?: (objectId: string, u: number, v: number) => void;
	readonly gumballDragActive: boolean;
	readonly onGumballDraggingChanged: (dragging: boolean) => void;
	readonly onGumballDragEnd: (kind: GumballHandleKind, before: GumballPose, after: GumballPose) => void;
	readonly previewComponentIds: ReadonlySet<number>;
}) {
	const meshById = useMemo(() => new Map(meshes.map((mesh) => [mesh.id, mesh])), [meshes]);
	const geometries = useMemo(() => {
		const map = new Map<string, BufferGeometry>();
		for (const mesh of meshes) {
			if (mesh.data) map.set(mesh.id, geometryFromMesh(mesh.data));
		}
		return map;
	}, [meshes]);
	const targets = selection.targets ?? { mesh: true, vertex: false, edge: false, face: false };
	const selectionMode = selection.selectionMode ?? selection.granularity ?? "mesh";
	const selectedComponentIds = new Set(selection.componentIds ?? []);
	const pickEnabled = !gumballDragActive && !onPaintAt;
	const transformTool = selection.transformTool ?? "move";
	const gumballConfig = useMemo(() => gumballConfigForTransformTool(transformTool), [transformTool]);
	const paintMode = selection.interactionMode === "paint";

	const mergeMode = (event: { shiftKey?: boolean; ctrlKey?: boolean; metaKey?: boolean }) => {
		const mode = marqueeModeFromModifiers(event);
		if (mode === "additive") return "add";
		if (mode === "subtractive" || mode === "invertive") return "toggle";
		return "replace";
	};

	const paintFromHit = (
		objectId: string,
		mesh: WorldMeshData,
		event: ThreeEvent<PointerEvent> & { faceIndex?: number | null; uv?: { x: number; y: number } },
	) => {
		if (!onPaintAt) return;
		let u = event.uv?.x;
		let v = event.uv?.y;
		if (u == null || v == null) {
			if (event.faceIndex == null || !mesh.indices.length) return;
			const i0 = mesh.indices[event.faceIndex * 3] ?? 0;
			const i1 = mesh.indices[event.faceIndex * 3 + 1] ?? 0;
			const i2 = mesh.indices[event.faceIndex * 3 + 2] ?? 0;
			if (!mesh.uvs || mesh.uvs.length < 6) return;
			u = (mesh.uvs[i0 * 2]! + mesh.uvs[i1 * 2]! + mesh.uvs[i2 * 2]!) / 3;
			v = (mesh.uvs[i0 * 2 + 1]! + mesh.uvs[i1 * 2 + 1]! + mesh.uvs[i2 * 2 + 1]!) / 3;
		}
		onPaintAt(objectId, u, v);
	};

	return (
		<WorldLayerStack>
			<group>
				{instances.map((instance, index) => {
					const meshId = instance.meshId ?? instance.id;
					const meshRecord = meshById.get(meshId);
					const meshData = meshRecord?.data;
					const geometry = geometries.get(meshId);
					const position = instance.position ?? [instance.x ?? index, instance.y ?? 0, instance.z ?? 0];
					const scale = instance.scale ?? [1, 1, 1];
					const rotation = instance.rotation;
					const quaternion = rotation
						? new Quaternion(rotation[0], rotation[1], rotation[2], rotation[3])
						: undefined;
					return (
						<WorldInstanceNode
							key={instance.id}
							instance={instance}
							index={index}
							meshRecord={meshRecord}
							meshData={meshData}
							geometry={geometry}
							colors={colors}
							vertexPick={meshData ? buildVertexPickData(meshData) : null}
							edgeGeometry={meshData ? buildEdgeGeometry(meshData) : null}
							paintTextureBase64={meshData?.paintTextureBase64}
							position={position as [number, number, number]}
							scale={scale as [number, number, number]}
							quaternion={quaternion}
							targets={targets}
							activeObjectId={selection.activeObjectId}
							selectionMode={selectionMode}
							selectedComponentIds={selectedComponentIds}
							previewComponentIds={previewComponentIds}
							hoveredComponent={selection.hoveredComponent}
							showEdges={selection.showEdges}
							pickEnabled={pickEnabled}
							onPaintAt={onPaintAt}
							paintFromHit={paintFromHit}
							flatShading={instance.smoothShading === false}
							onInstancePointerDown={onInstancePointerDown}
							onInstancePointerMove={onInstancePointerMove}
							onWorldPick={onWorldPick}
							onComponentHover={onComponentHover}
							mergeMode={mergeMode}
						/>
					);
				})}
			</group>
			<SceneGumball
				target={selection.gumballTarget}
				config={gumballConfig}
				active={Boolean(selection.gumballActive) && !paintMode}
				onDraggingChanged={onGumballDraggingChanged}
				onDragEnd={onGumballDragEnd}
			/>
		</WorldLayerStack>
	);
}
//#endregion WorldInstancesLayer

function pointInMarqueeRect(sx: number, sy: number, marquee: readonly SelectionMarqueePoint[]): boolean {
	if (marquee.length < 2) return false;
	const start = marquee[0]!;
	const end = marquee[marquee.length - 1]!;
	const minX = Math.min(start.x, end.x);
	const maxX = Math.max(start.x, end.x);
	const minY = Math.min(start.y, end.y);
	const maxY = Math.max(start.y, end.y);
	return sx >= minX && sx <= maxX && sy >= minY && sy <= maxY;
}

function projectWorldPoint(
	point: readonly [number, number, number],
	offset: readonly [number, number, number],
	camera: import("three").Camera,
	rect: DOMRect,
): { readonly x: number; readonly y: number } {
	const projected = new Vector3(point[0] + offset[0], point[1] + offset[1], point[2] + offset[2]).project(camera);
	return {
		x: ((projected.x + 1) / 2) * rect.width,
		y: ((-projected.y + 1) / 2) * rect.height,
	};
}

function resolveMarqueeComponentIds(
	instances: readonly WorldInstanceRecord[],
	meshes: readonly WorldMeshRecord[],
	selectionMode: string,
	activeObjectId: string | undefined,
	marquee: readonly SelectionMarqueePoint[],
	rect: DOMRect,
	camera: import("three").Camera,
): readonly number[] {
	const active = instances.find((instance) => instance.id === activeObjectId);
	if (!active) return [];
	const meshId = active.meshId ?? active.id;
	const meshData = meshes.find((mesh) => mesh.id === meshId)?.data;
	if (!meshData) return [];
	const offset = (active.position ?? [0, 0, 0]) as [number, number, number];
	const hits = new Set<number>();
	if (selectionMode === "vertex") {
		const pick = buildVertexPickData(meshData);
		if (!pick) return [];
		const positions = pick.geometry.attributes.position!;
		for (let index = 0; index < pick.vertexIds.length; index += 1) {
			const screen = projectWorldPoint(
				[positions.getX(index), positions.getY(index), positions.getZ(index)],
				offset,
				camera,
				rect,
			);
			if (pointInMarqueeRect(screen.x, screen.y, marquee)) hits.add(pick.vertexIds[index]!);
		}
	} else if (selectionMode === "edge" && meshData.edgeIds && meshData.edgePositions) {
		for (let edgeIndex = 0; edgeIndex < meshData.edgeIds.length; edgeIndex += 1) {
			const base = edgeIndex * 6;
			const screen = projectWorldPoint(
				[
					(meshData.edgePositions[base]! + meshData.edgePositions[base + 3]!) * 0.5,
					(meshData.edgePositions[base + 1]! + meshData.edgePositions[base + 4]!) * 0.5,
					(meshData.edgePositions[base + 2]! + meshData.edgePositions[base + 5]!) * 0.5,
				],
				offset,
				camera,
				rect,
			);
			if (pointInMarqueeRect(screen.x, screen.y, marquee)) hits.add(meshData.edgeIds[edgeIndex]!);
		}
	} else if (selectionMode === "face" && meshData.faceIds && meshData.indices.length) {
		for (let faceIndex = 0; faceIndex < meshData.faceIds.length; faceIndex += 1) {
			const i0 = meshData.indices[faceIndex * 3] ?? 0;
			const i1 = meshData.indices[faceIndex * 3 + 1] ?? 0;
			const i2 = meshData.indices[faceIndex * 3 + 2] ?? 0;
			const screen = projectWorldPoint(
				[
					(meshData.positions[i0 * 3]! + meshData.positions[i1 * 3]! + meshData.positions[i2 * 3]!) / 3,
					(meshData.positions[i0 * 3 + 1]! + meshData.positions[i1 * 3 + 1]! + meshData.positions[i2 * 3 + 1]!) / 3,
					(meshData.positions[i0 * 3 + 2]! + meshData.positions[i1 * 3 + 2]! + meshData.positions[i2 * 3 + 2]!) / 3,
				],
				offset,
				camera,
				rect,
			);
			if (pointInMarqueeRect(screen.x, screen.y, marquee)) hits.add(meshData.faceIds[faceIndex]!);
		}
	}
	return [...hits];
}

function CameraRefBridge({ cameraRef }: { readonly cameraRef: React.MutableRefObject<import("three").Camera | null> }) {
	const camera = useThree((state) => state.camera);
	useEffect(() => {
		cameraRef.current = camera;
	}, [camera, cameraRef]);
	return null;
}

function paneSuffixFromSurfaceId(surfaceId?: string): string | undefined {
	if (!surfaceId) return undefined;
	const slash = surfaceId.lastIndexOf("/");
	return slash >= 0 ? surfaceId.slice(slash + 1) : surfaceId;
}

function raycastGroundPoint(
	clientX: number,
	clientY: number,
	hostRect: DOMRect,
	camera: import("three").Camera,
): [number, number, number] | null {
	const ndcX = ((clientX - hostRect.left) / hostRect.width) * 2 - 1;
	const ndcY = -(((clientY - hostRect.top) / hostRect.height) * 2 - 1);
	const ray = new Vector3(ndcX, ndcY, 0.5).unproject(camera);
	const origin = camera.position.clone();
	const direction = ray.sub(origin).normalize();
	if (Math.abs(direction.z) < 1e-6) return null;
	const t = -origin.z / direction.z;
	if (t < 0) return null;
	const hit = origin.add(direction.multiplyScalar(t));
	return [hit.x, hit.y, hit.z];
}

function instanceCenterInMarquee(
	instance: WorldInstanceRecord,
	index: number,
	marquee: readonly SelectionMarqueePoint[],
	hostRect: DOMRect,
	camera: import("three").Camera,
): boolean {
	if (marquee.length < 2) return false;
	const position = instance.position ?? [instance.x ?? index, instance.y ?? 0, instance.z ?? 0];
	const projected = new Vector3(position[0], position[1], position[2]).project(camera);
	const sx = ((projected.x + 1) / 2) * hostRect.width;
	const sy = ((-projected.y + 1) / 2) * hostRect.height;
	const start = marquee[0]!;
	const end = marquee[marquee.length - 1]!;
	const minX = Math.min(start.x, end.x);
	const maxX = Math.max(start.x, end.x);
	const minY = Math.min(start.y, end.y);
	const maxY = Math.max(start.y, end.y);
	return sx >= minX && sx <= maxX && sy >= minY && sy <= maxY;
}

//#region World3dHost
export function World3dHost({
	node,
	onCommand,
}: {
	readonly node: UiComponentSceneNode;
	readonly onCommand: (command: CommandDescriptor) => void;
}) {
	const scene = node.world3d;
	const colors = useSemanticColors();
	const cameraState = useMemo(() => parseCameraState(scene?.cameraJson ?? "{}"), [scene?.cameraJson]);
	const meshes = useMemo(() => parseMeshes(scene?.meshesJson ?? "[]"), [scene?.meshesJson]);
	const instances = useMemo(() => parseInstances(scene?.instancesJson ?? "[]"), [scene?.instancesJson]);
	const selection = useMemo(() => parseSelection(scene?.selectionJson ?? "{}"), [scene?.selectionJson]);
	const hostRef = useRef<HTMLDivElement | null>(null);
	const lodRef = useRef(DEFAULT_MANUAL_LOD);
	const [marqueePath, setMarqueePath] = useState<readonly SelectionMarqueePoint[]>([]);
	const [marqueeActive, setMarqueeActive] = useState(false);
	const [marqueePreviewIds, setMarqueePreviewIds] = useState<readonly number[]>([]);
	const [gumballDragActive, setGumballDragActive] = useState(false);
	const [paintStrokeActive, setPaintStrokeActive] = useState(false);
	const cameraRef = useRef<import("three").Camera | null>(null);
	const selectionMode = selection.selectionMode ?? selection.granularity ?? "mesh";

	const dispatch = useCallback(
		(command: string, args?: Record<string, unknown>) => {
			onCommand({
				controllerId: node.controllerId,
				command,
				args: { surfaceId: node.surfaceId, ...args },
			});
		},
		[node.controllerId, node.surfaceId, onCommand],
	);

	const mergeModeToArg = (mode: ReturnType<typeof marqueeModeFromModifiers>) => {
		if (mode === "additive") return "add";
		if (mode === "subtractive" || mode === "invertive") return "toggle";
		return "replace";
	};

	const selectionArgs = useCallback(
		() => ({
			mode: selection.selectionMode ?? selection.granularity ?? "mesh",
			ids: selection.componentIds ?? [],
		}),
		[selection.componentIds, selection.granularity, selection.selectionMode],
	);

	const handleInstancePointerDown = useCallback(
		(id: string, index: number, event: { shiftKey: boolean; ctrlKey: boolean; metaKey: boolean }) => {
			const merge = mergeModeToArg(marqueeModeFromModifiers(event));
			if (selectionMode === "mesh" || selectionMode === "object") {
				dispatch("worldPick", { granularity: "mesh", id: index, merge });
				return;
			}
			dispatch("worldSelect", {
				ids: [id],
				merge,
			});
		},
		[dispatch, selectionMode],
	);

	const handleInstancePointerMove = useCallback(
		(id: string | null) => {
			if (id == null) {
				dispatch("setHover", {});
				return;
			}
			dispatch("setHover", { objectId: id, mode: "mesh", id: 0 });
		},
		[dispatch],
	);

	const handleComponentHover = useCallback(
		(args: { objectId: string; mode: string; id: number } | null) => {
			if (!args) {
				dispatch("setHover", {});
				return;
			}
			dispatch("setHover", args);
		},
		[dispatch],
	);

	const handleWorldPick = useCallback(
		(args: { granularity: string; id: number; merge: string }) => {
			dispatch("worldPick", args);
		},
		[dispatch],
	);

	const paintMode = selection.interactionMode === "paint";
	const handlePaintAt = useCallback(
		(objectId: string, u: number, v: number) => {
			dispatch("paintAt", { objectId, u, v });
		},
		[dispatch],
	);

	const handleCameraChange = useCallback(
		(state: WorldCameraState) => {
			dispatch("setCamera", {
				camera: {
					position: state.position,
					target: state.target,
					fov: cameraState.fov,
				},
			});
		},
		[cameraState.fov, dispatch],
	);

	const updateMarqueePreview = useCallback(
		(path: readonly SelectionMarqueePoint[]) => {
			if (path.length < 2 || !hostRef.current || !cameraRef.current) {
				setMarqueePreviewIds([]);
				return;
			}
			if (selectionMode === "mesh" || selectionMode === "object") {
				setMarqueePreviewIds([]);
				return;
			}
			const rect = hostRef.current.getBoundingClientRect();
			setMarqueePreviewIds(
				resolveMarqueeComponentIds(
					instances,
					meshes,
					selectionMode,
					selection.activeObjectId,
					path,
					rect,
					cameraRef.current,
				),
			);
		},
		[instances, meshes, selection.activeObjectId, selectionMode],
	);

	const handleGumballDragEnd = useCallback(
		(_kind: GumballHandleKind, before: GumballPose, after: GumballPose) => {
			const tool =
				selection.transformTool === "rotate"
					? "rotate"
					: selection.transformTool === "scale"
						? "scale"
						: "translate";
			const base = selectionArgs();
			if (tool === "translate") {
				dispatch("translateSelection", {
					...base,
					dx: after.position[0] - before.position[0],
					dy: after.position[1] - before.position[1],
					dz: after.position[2] - before.position[2],
				});
				return;
			}
			if (tool === "rotate") {
				dispatch("rotateSelection", {
					...base,
					ax: 0,
					ay: 1,
					az: 0,
					angle: after.rotation[1] - before.rotation[1],
				});
				return;
			}
			const sx = after.scale[0] / Math.max(before.scale[0], 1e-6);
			const sy = after.scale[1] / Math.max(before.scale[1], 1e-6);
			const sz = after.scale[2] / Math.max(before.scale[2], 1e-6);
			dispatch("scaleSelection", { ...base, sx, sy, sz });
		},
		[dispatch, selection.transformTool, selectionArgs],
	);

	const toLocalPoint = useCallback((event: React.PointerEvent<HTMLDivElement>): SelectionMarqueePoint => {
		const rect = hostRef.current?.getBoundingClientRect();
		if (!rect) return { x: event.clientX, y: event.clientY };
		return { x: event.clientX - rect.left, y: event.clientY - rect.top };
	}, []);

	const handlePointerDown = useCallback(
		(event: React.PointerEvent<HTMLDivElement>) => {
			if (event.button !== 0 || event.target !== hostRef.current) return;
			if (selection.engagementSessionActive && hostRef.current && cameraRef.current) {
				const rect = hostRef.current.getBoundingClientRect();
				const point = raycastGroundPoint(event.clientX, event.clientY, rect, cameraRef.current);
				if (point) {
					dispatch("worldPointerDown", {
						pane: paneSuffixFromSurfaceId(node.surfaceId),
						position: point,
						shiftKey: event.shiftKey,
						ctrlKey: event.ctrlKey,
						metaKey: event.metaKey,
					});
					return;
				}
			}
			if (paintMode) {
				setPaintStrokeActive(true);
				dispatch("paintStrokeBegin");
			}
			setMarqueeActive(true);
			const point = toLocalPoint(event);
			setMarqueePath([point]);
			updateMarqueePreview([point]);
		},
		[
			dispatch,
			node.surfaceId,
			paintMode,
			selection.engagementSessionActive,
			toLocalPoint,
			updateMarqueePreview,
		],
	);

	const handlePointerMove = useCallback(
		(event: React.PointerEvent<HTMLDivElement>) => {
			if (!marqueeActive) return;
			setMarqueePath((path) => {
				const next = [...path, toLocalPoint(event)];
				updateMarqueePreview(next);
				return next;
			});
		},
		[marqueeActive, toLocalPoint, updateMarqueePreview],
	);

	const handlePointerUp = useCallback(
		(event: React.PointerEvent<HTMLDivElement>) => {
			if (marqueeActive && marqueePath.length === 1 && event.target === hostRef.current) {
				dispatch("worldPick", {
					granularity: selectionMode,
					id: null,
					merge: mergeModeToArg(marqueeModeFromModifiers(event)),
				});
			}
			if (marqueeActive && marqueePath.length > 1 && hostRef.current && cameraRef.current) {
				const rect = hostRef.current.getBoundingClientRect();
				const camera = cameraRef.current;
				const merge = mergeModeToArg(marqueeModeFromModifiers(event));
				if (selectionMode === "mesh" || selectionMode === "object") {
					const selected = instances
						.filter((instance, index) => instanceCenterInMarquee(instance, index, marqueePath, rect, camera))
						.map((instance) => instance.id);
					if (selected.length > 0) {
						dispatch("worldSelect", { ids: selected, merge });
					}
				} else {
					const hits = resolveMarqueeComponentIds(
						instances,
						meshes,
						selectionMode,
						selection.activeObjectId,
						marqueePath,
						rect,
						camera,
					);
					if (hits.length > 0) {
						const ids =
							merge === "add"
								? [...new Set([...(selection.componentIds ?? []), ...hits])]
								: merge === "toggle"
									? hits
									: hits;
						if (merge === "toggle") {
							for (const id of hits) {
								dispatch("worldPick", { granularity: selectionMode, id, merge: "toggle" });
							}
						} else {
							dispatch("setSelection", { mode: selectionMode, ids });
						}
					}
				}
			}
			if (paintStrokeActive) {
				dispatch("paintStrokeEnd");
				setPaintStrokeActive(false);
			}
			setMarqueeActive(false);
			setMarqueePath([]);
			setMarqueePreviewIds([]);
		},
		[
			dispatch,
			instances,
			marqueeActive,
			marqueePath,
			meshes,
			paintStrokeActive,
			selection.activeObjectId,
			selection.componentIds,
			selectionMode,
		],
	);

	if (!scene) return <div className="semio-world-3d-empty">No world scene</div>;

	const method = selection.method ?? "rectangle";
	const marqueeStart = marqueePath[0];
	const marqueeEnd = marqueePath[marqueePath.length - 1];
	const marqueeCoverage =
		marqueeStart && marqueeEnd
			? marqueeCoverageFromGesture({
					method,
					startX: marqueeStart.x,
					endX: marqueeEnd.x,
					path: marqueePath,
				})
			: "full";

	return (
		<div
			ref={hostRef}
			className="semio-world-3d-host relative h-full min-h-[24rem] w-full"
			data-surface-id={node.surfaceId}
			onPointerDown={handlePointerDown}
			onPointerMove={handlePointerMove}
			onPointerUp={handlePointerUp}
		>
			<WorldCanvas className="h-full w-full" cameraUp={[0, 0, 1]} cameraFov={cameraState.fov}>
				<WorldOrbitViewSnapGateProvider>
					<WorldOrbitCameraViewRig
						state={cameraState}
						seedKey={scene?.cameraJson ?? "default"}
						perspectiveFov={cameraState.fov}
					/>
					<WorldOrbitGated
						controlsGate={marqueeActive || gumballDragActive}
						onCamera={handleCameraChange}
					/>
					<WorldLodBridge
					lodRef={lodRef}
					distanceReference={100}
					gridFactor={DEFAULT_LOD_GRID_FACTOR}
					gridSnapEnabled={false}
					showLodGrid
					automaticLod
					depthVariableLod={false}
					manualLod={DEFAULT_MANUAL_LOD}
					gridDatum={[0, 0, 0]}
				>
					<ambientLight intensity={0.65} />
					<directionalLight intensity={0.85} position={[4, 6, 8]} />
					<CameraRefBridge cameraRef={cameraRef} />
					<WorldInstancesLayer
						instances={instances}
						meshes={meshes}
						selection={selection}
						colors={colors}
						onInstancePointerDown={handleInstancePointerDown}
						onInstancePointerMove={handleInstancePointerMove}
						onWorldPick={handleWorldPick}
						onComponentHover={handleComponentHover}
						onPaintAt={paintMode ? handlePaintAt : undefined}
						gumballDragActive={gumballDragActive}
						onGumballDraggingChanged={setGumballDragActive}
						onGumballDragEnd={handleGumballDragEnd}
						previewComponentIds={new Set(marqueePreviewIds)}
					/>
				</WorldLodBridge>
				</WorldOrbitViewSnapGateProvider>
			</WorldCanvas>
			{marqueeActive && marqueePath.length > 1 && marqueeStart && marqueeEnd ? (
				method === "lasso" ? (
					<SelectionMarquee coverage={marqueeCoverage} shape="polygon" points={marqueePath} />
				) : (
					<SelectionMarquee
						coverage={marqueeCoverage}
						shape="rect"
						rect={{
							x: Math.min(marqueeStart.x, marqueeEnd.x),
							y: Math.min(marqueeStart.y, marqueeEnd.y),
							width: Math.abs(marqueeEnd.x - marqueeStart.x),
							height: Math.abs(marqueeEnd.y - marqueeStart.y),
						}}
					/>
				)
			) : null}
		</div>
	);
}
//#endregion World3dHost
