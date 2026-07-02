// #region 🧲Header
/// <reference types="vite/client" />
/// <reference types="vitest/importMeta" />
/** @emoji 🔷 `@semio-tech/lowpoly-react` — low-poly mesh editor viewport. */
// #endregion 🧲Header

import type {
	LowpolyPaintTool,
	LowpolySceneObject,
	LowpolySelectionMode,
	LowpolyTarget,
	LowpolyTessellation,
	LowpolyTransform,
} from "@semio-tech/lowpoly-core";
import { isLowpolyFixtureReady } from "@semio-tech/lowpoly-core";
import {
	DEFAULT_LOD_GRID_FACTOR,
	DEFAULT_MANUAL_LOD,
	WorldCameraInvalidator,
	WorldCanvas,
	WorldLayer,
	WorldLodBridge,
	WorldOrbitCameraViewRig,
	WorldOrbitGated,
	WorldOrbitProjectionSwitch,
	WorldOrbitViewControls,
	WorldOrbitViewSnapGateProvider,
	applyOrbitProjectionToCameraState,
	type OrbitCameraProjection,
	type WorldCameraState,
} from "@semio-tech/infinite-world-r3f";
import {
	SelectionMarquee,
	UnifiedGumball,
	canvasHostRootClass,
	cn,
	gumballPointerConsumesCanvasEventRef,
	marqueeCoverageFromGesture,
	marqueeModeFromModifiers,
	reactHostPort,
	sceneHostPort,
	selectionMergeIds,
	type GumballHandleKind,
	type GumballPose,
	type SelectionMarqueeCoverage,
	type SelectionMergeMode,
} from "@semio-tech/ui-react";
import { clearColorResolveCache, resolveSemanticColorHex } from "@semio-tech/ui-styling";

const THREE = sceneHostPort.three;
const LOWPOLY_MARQUEE_THRESHOLD_PX = 4;

//#region WasmBridge

export type LowpolySessionWasm = {
	fixtureJson(): string;
	loadFixtureJson(json: string): void;
	addPrimitive(kind: string): string;
	setActiveObject(id: string): void;
	setSelection(mode: string, ids: number[]): void;
	tessellateActive(): string;
	tessellateAll(): string;
	exportObjActive(): string;
	extrudeFaces(distance: number): void;
	insetFaces(amount: number): void;
	flipFaces(faceIds: number[]): void;
	bevelEdges(amount: number, segments: number): void;
	loopCut(cuts: number): void;
	mergeVertices(): void;
	dissolveEdges(): void;
	subdivideFaces(): void;
	triangulate(): void;
	mirror(axis: string, weldThreshold: number): void;
	decimate(targetRatio: number): void;
	translateSelection(dx: number, dy: number, dz: number): void;
	rotateSelection(ax: number, ay: number, az: number, angle: number): void;
	scaleSelection(sx: number, sy: number, sz: number): void;
	snapToGrid(grid: number): void;
	setSmoothShading(smooth: boolean): void;
	markUvSeam(seam: boolean, edgeIds: number[]): void;
	unwrapActive(): void;
	compositePaintTexture(objectId: string): Uint8Array;
	paintLayerPixels(objectId: string, layerIndex: number): Uint8Array;
	setPaintLayerPixels(objectId: string, layerIndex: number, pixels: Uint8Array): void;
	paintStroke(
		objectId: string,
		layerIndex: number,
		u: number,
		v: number,
		radius: number,
		r: number,
		g: number,
		b: number,
		a: number,
		hardness: number,
		opacity: number,
		eraser: boolean,
	): void;
	fillBucket(objectId: string, layerIndex: number, u: number, v: number, r: number, g: number, b: number, a: number): void;
	samplePixel(objectId: string, u: number, v: number): Uint8Array;
	addPaintLayer(objectId: string, name: string): number;
	removePaintLayer(objectId: string, layerIndex: number): void;
	setLayerVisible(objectId: string, layerIndex: number, visible: boolean): void;
	setLayerOpacity(objectId: string, layerIndex: number, opacity: number): void;
};

let lowpolySessionPromise: Promise<typeof import("../../core/rs/pkg/lowpoly_core.js")> | null = null;

export async function ensureLowpolyWasm(): Promise<typeof import("../../core/rs/pkg/lowpoly_core.js")> {
	lowpolySessionPromise ??= import("../../core/rs/pkg/lowpoly_core.js");
	return lowpolySessionPromise;
}

export async function loadDefaultLowpolyFixtureJson(): Promise<string> {
	const wasm = await ensureLowpolyWasm();
	await wasm.default();
	return wasm.defaultFixtureJson();
}

export async function createLowpolySession(fixtureJson?: string): Promise<LowpolySessionWasm> {
	const wasm = await ensureLowpolyWasm();
	await wasm.default();
	return new wasm.LowpolySession(fixtureJson ?? undefined) as unknown as LowpolySessionWasm;
}

function toUint32Array(values: number[] | undefined): Uint32Array {
	return new Uint32Array(values ?? []);
}

export function tessellationFromWasm(raw: {
	positions: number[];
	normals: number[];
	indices: number[];
	edgePositions: number[];
	faceIds?: number[];
	vertexIds?: number[];
	edgeIds?: number[];
	edgeUvs?: number[];
	edgeIsSeam?: number[];
	uvs?: number[];
}): LowpolyTessellation {
	return {
		positions: new Float32Array(raw.positions),
		normals: new Float32Array(raw.normals),
		indices: new Uint32Array(raw.indices),
		edgePositions: new Float32Array(raw.edgePositions),
		faceIds: toUint32Array(raw.faceIds),
		vertexIds: toUint32Array(raw.vertexIds),
		edgeIds: toUint32Array(raw.edgeIds),
		uvs: new Float32Array(raw.uvs ?? []),
		edgeUvs: new Float32Array(raw.edgeUvs ?? []),
		edgeIsSeam: new Uint8Array(raw.edgeIsSeam ?? []),
	};
}

export function parseLowpolyTessellationJson(json: string): LowpolyTessellation | null {
	try {
		const raw = JSON.parse(json) as Parameters<typeof tessellationFromWasm>[0];
		if (!raw.positions?.length || !raw.indices?.length) return null;
		return tessellationFromWasm(raw);
	} catch {
		return null;
	}
}

export function parseLowpolySceneJson(json: string): LowpolySceneObject[] {
	try {
		const raw = JSON.parse(json) as Array<{
			id: string;
			index: number;
			name: string;
			transform: LowpolyTransform;
			smoothShading: boolean;
			active: boolean;
			tessellation: Parameters<typeof tessellationFromWasm>[0];
		}>;
		return raw.map((entry) => ({
			id: entry.id,
			index: entry.index,
			name: entry.name,
			transform: entry.transform,
			smoothShading: entry.smoothShading,
			active: entry.active,
			tessellation: tessellationFromWasm(entry.tessellation),
		}));
	} catch {
		return [];
	}
}

export function safeLoadLowpolyFixture(session: LowpolySessionWasm, json: string): boolean {
	if (!isLowpolyFixtureReady(json)) return false;
	try {
		session.loadFixtureJson(json);
		return true;
	} catch {
		return false;
	}
}

export function tessellateLowpolySession(session: LowpolySessionWasm): LowpolyTessellation | null {
	try {
		return parseLowpolyTessellationJson(session.tessellateActive());
	} catch {
		return null;
	}
}

export function tessellateAllLowpolySession(session: LowpolySessionWasm): LowpolySceneObject[] {
	try {
		return parseLowpolySceneJson(session.tessellateAll());
	} catch {
		return [];
	}
}

//#endregion WasmBridge

//#region MeshBuffers

function eulerToQuaternion(rotation: [number, number, number]): THREE.Quaternion {
	const euler = new THREE.Euler(rotation[0], rotation[1], rotation[2], "XYZ");
	return new THREE.Quaternion().setFromEuler(euler);
}

function buildMeshGeometry(tess: LowpolyTessellation): {
	surface: THREE.BufferGeometry | null;
	edges: THREE.BufferGeometry | null;
} {
	let surface: THREE.BufferGeometry | null = null;
	if (tess.positions.length > 0 && tess.indices.length > 0) {
		const geometry = new THREE.BufferGeometry();
		geometry.setAttribute("position", new THREE.Float32BufferAttribute(tess.positions, 3));
		geometry.setAttribute("normal", new THREE.Float32BufferAttribute(tess.normals, 3));
		if (tess.uvs.length > 0) {
			geometry.setAttribute("uv", new THREE.Float32BufferAttribute(tess.uvs, 2));
		}
		geometry.setIndex(new THREE.BufferAttribute(tess.indices, 1));
		surface = geometry;
	}
	let edges: THREE.BufferGeometry | null = null;
	if (tess.edgePositions.length > 0) {
		const geometry = new THREE.BufferGeometry();
		geometry.setAttribute("position", new THREE.Float32BufferAttribute(tess.edgePositions, 3));
		edges = geometry;
	}
	return { surface, edges };
}

function buildFaceOverlayGeometry(tessellation: LowpolyTessellation, ids: ReadonlySet<number>): THREE.BufferGeometry | null {
	if (!ids.size) return null;
	const positions: number[] = [];
	for (let triangle = 0; triangle < tessellation.faceIds.length; triangle += 1) {
		if (!ids.has(tessellation.faceIds[triangle]!)) continue;
		for (let corner = 0; corner < 3; corner += 1) {
			const vertex = tessellation.indices[triangle * 3 + corner];
			if (vertex == null) continue;
			positions.push(
				tessellation.positions[vertex * 3]!,
				tessellation.positions[vertex * 3 + 1]!,
				tessellation.positions[vertex * 3 + 2]!,
			);
		}
	}
	if (!positions.length) return null;
	const geometry = new THREE.BufferGeometry();
	geometry.setAttribute("position", new THREE.Float32BufferAttribute(positions, 3));
	return geometry;
}

function buildEdgeOverlayGeometry(tessellation: LowpolyTessellation, ids: ReadonlySet<number>): THREE.BufferGeometry | null {
	if (!ids.size) return null;
	const positions: number[] = [];
	for (let edge = 0; edge < tessellation.edgeIds.length; edge += 1) {
		if (!ids.has(tessellation.edgeIds[edge]!)) continue;
		positions.push(...tessellation.edgePositions.slice(edge * 6, edge * 6 + 6));
	}
	if (!positions.length) return null;
	const geometry = new THREE.BufferGeometry();
	geometry.setAttribute("position", new THREE.Float32BufferAttribute(positions, 3));
	return geometry;
}

function buildVertexPickGeometry(tessellation: LowpolyTessellation): THREE.BufferGeometry | null {
	if (!tessellation.positions.length) return null;
	const positions: number[] = [];
	const emitted = new Set<number>();
	for (let index = 0; index < tessellation.vertexIds.length; index += 1) {
		const id = tessellation.vertexIds[index]!;
		if (emitted.has(id)) continue;
		emitted.add(id);
		positions.push(
			tessellation.positions[index * 3]!,
			tessellation.positions[index * 3 + 1]!,
			tessellation.positions[index * 3 + 2]!,
		);
	}
	if (!positions.length) return null;
	const geometry = new THREE.BufferGeometry();
	geometry.setAttribute("position", new THREE.Float32BufferAttribute(positions, 3));
	return geometry;
}

function buildVertexOverlayGeometry(tessellation: LowpolyTessellation, ids: ReadonlySet<number>): THREE.BufferGeometry | null {
	if (!ids.size) return null;
	const positions: number[] = [];
	const emitted = new Set<number>();
	for (let index = 0; index < tessellation.vertexIds.length; index += 1) {
		const id = tessellation.vertexIds[index]!;
		if (!ids.has(id) || emitted.has(id)) continue;
		emitted.add(id);
		positions.push(
			tessellation.positions[index * 3]!,
			tessellation.positions[index * 3 + 1]!,
			tessellation.positions[index * 3 + 2]!,
		);
	}
	if (!positions.length) return null;
	const geometry = new THREE.BufferGeometry();
	geometry.setAttribute("position", new THREE.Float32BufferAttribute(positions, 3));
	return geometry;
}

function meshCentroid(positions: Float32Array): [number, number, number] {
	if (positions.length < 3) return [0, 0, 0];
	let x = 0;
	let y = 0;
	let z = 0;
	const count = positions.length / 3;
	for (let i = 0; i < positions.length; i += 3) {
		x += positions[i] ?? 0;
		y += positions[i + 1] ?? 0;
		z += positions[i + 2] ?? 0;
	}
	return [x / count, y / count, z / count];
}

function boundsFromPositions(positions: Float32Array): { min: THREE.Vector3; max: THREE.Vector3 } | null {
	if (positions.length < 3) return null;
	const min = new THREE.Vector3(Number.POSITIVE_INFINITY, Number.POSITIVE_INFINITY, Number.POSITIVE_INFINITY);
	const max = new THREE.Vector3(Number.NEGATIVE_INFINITY, Number.NEGATIVE_INFINITY, Number.NEGATIVE_INFINITY);
	for (let i = 0; i < positions.length; i += 3) {
		min.x = Math.min(min.x, positions[i] ?? 0);
		min.y = Math.min(min.y, positions[i + 1] ?? 0);
		min.z = Math.min(min.z, positions[i + 2] ?? 0);
		max.x = Math.max(max.x, positions[i] ?? 0);
		max.y = Math.max(max.y, positions[i + 1] ?? 0);
		max.z = Math.max(max.z, positions[i + 2] ?? 0);
	}
	return { min, max };
}

function projectWorldPoint(camera: THREE.Camera, size: { width: number; height: number }, point: THREE.Vector3): { x: number; y: number } | null {
	const vector = point.clone().project(camera);
	if (!Number.isFinite(vector.x) || !Number.isFinite(vector.y)) return null;
	return {
		x: ((vector.x + 1) / 2) * size.width,
		y: ((-vector.y + 1) / 2) * size.height,
	};
}

function screenRectContainsPoint(rect: { left: number; top: number; right: number; bottom: number }, point: { x: number; y: number }, crossing: boolean): boolean {
	if (crossing) {
		return point.x >= Math.min(rect.left, rect.right) && point.x <= Math.max(rect.left, rect.right) && point.y >= Math.min(rect.top, rect.bottom) && point.y <= Math.max(rect.top, rect.bottom);
	}
	const left = Math.min(rect.left, rect.right);
	const right = Math.max(rect.left, rect.right);
	const top = Math.min(rect.top, rect.bottom);
	const bottom = Math.max(rect.top, rect.bottom);
	return point.x >= left && point.x <= right && point.y >= top && point.y <= bottom;
}

//#endregion MeshBuffers

//#region LowpolyCanvas

export type LowpolyTransformTool = "move" | "rotate" | "scale";

export interface LowpolyCanvasProps {
	readonly fixtureJson: string;
	readonly sceneObjects: readonly LowpolySceneObject[];
	readonly selectionMode: LowpolySelectionMode;
	readonly selectedIds: readonly number[];
	readonly hoveredTarget?: LowpolyTarget | null;
	readonly transformTool: LowpolyTransformTool;
	readonly session: LowpolySessionWasm | null;
	readonly interactionMode?: "model" | "paint";
	readonly paintTool?: LowpolyPaintTool;
	readonly paintLayerIndex?: number;
	readonly paintColor?: readonly [number, number, number, number];
	readonly paintBrushSize?: number;
	readonly paintBrushOpacity?: number;
	readonly paintBrushHardness?: number;
	readonly className?: string;
	readonly onFixtureChange?: (json: string) => void;
	readonly onSelectionChange?: (mode: LowpolySelectionMode, ids: readonly number[], activeObjectId?: string) => void;
	readonly onHoverChange?: (target: LowpolyTarget | null) => void;
	readonly onSceneChange?: (objects: readonly LowpolySceneObject[]) => void;
	readonly onPaintStrokeBegin?: () => void;
	readonly onPaintStrokeEnd?: () => void;
	readonly paintTextureRevision?: number;
	readonly onPaintTextureRefresh?: () => void;
}

function LowpolySceneInvalidator({ token }: { readonly token: number }): null {
	const invalidate = sceneHostPort.fiber.useThree((state) => state.invalidate);
	reactHostPort.useEffect(() => {
		invalidate();
	}, [invalidate, token]);
	return null;
}

function LowpolyLineRaycastThreshold(): null {
	const raycaster = sceneHostPort.fiber.useThree((state) => state.raycaster);
	reactHostPort.useEffect(() => {
		raycaster.params.Line.threshold = 0.08;
	}, [raycaster]);
	return null;
}

function LowpolyCameraBridge({
	onCamera,
}: {
	readonly onCamera: (camera: THREE.Camera, size: { width: number; height: number }) => void;
}): null {
	const camera = sceneHostPort.fiber.useThree((state) => state.camera);
	const size = sceneHostPort.fiber.useThree((state) => state.size);
	reactHostPort.useEffect(() => {
		onCamera(camera, size);
	}, [camera, onCamera, size, size.height, size.width]);
	return null;
}

function LowpolyMeshLayer({
	object,
	selectedIds,
	previewAddIds,
	previewRemoveIds,
	hoveredTarget,
	selectionMode,
	meshColor,
	edgeColor,
	selectColor,
	hoverColor,
	paintTexture,
	pickEnabled,
	onPick,
	onHover,
	onPaintAt,
}: {
	readonly object: LowpolySceneObject;
	readonly selectedIds: readonly number[];
	readonly previewAddIds: readonly number[];
	readonly previewRemoveIds: readonly number[];
	readonly hoveredTarget: LowpolyTarget | null;
	readonly selectionMode: LowpolySelectionMode;
	readonly meshColor: string;
	readonly edgeColor: string;
	readonly selectColor: string;
	readonly hoverColor: string;
	readonly paintTexture: THREE.Texture | null;
	readonly pickEnabled: boolean;
	readonly onPick: (objectIndex: number, id: number, mode: SelectionMergeMode) => void;
	readonly onHover: (target: LowpolyTarget | null) => void;
	readonly onPaintAt?: (objectId: string, u: number, v: number) => void;
}): React.ReactElement | null {
	const tessellation = object.tessellation;
	const { surface, edges } = reactHostPort.useMemo(() => buildMeshGeometry(tessellation), [tessellation]);
	const isObjectSelected = selectionMode === "object" && (selectedIds.includes(object.index) || previewAddIds.includes(object.index)) && !previewRemoveIds.includes(object.index);
	const hoveredId = hoveredTarget?.objectId === object.id && hoveredTarget.mode === selectionMode ? hoveredTarget.id : null;
	const isObjectHovered = selectionMode === "object" && (hoveredId === object.index || previewRemoveIds.includes(object.index));
	const selectedSet = reactHostPort.useMemo(() => {
		const next = new Set(selectedIds);
		for (const id of previewAddIds) next.add(id);
		for (const id of previewRemoveIds) next.delete(id);
		return next;
	}, [previewAddIds, previewRemoveIds, selectedIds]);
	const selectedFaceGeometry = reactHostPort.useMemo(
		() => selectionMode === "face" ? buildFaceOverlayGeometry(tessellation, selectedSet) : null,
		[selectedSet, selectionMode, tessellation],
	);
	const hoveredFaceGeometry = reactHostPort.useMemo(
		() => selectionMode === "face"
			? buildFaceOverlayGeometry(
				tessellation,
				new Set([
					...(hoveredId != null && !selectedSet.has(hoveredId) ? [hoveredId] : []),
					...previewRemoveIds.filter((id) => !selectedSet.has(id)),
				]),
			)
			: null,
		[hoveredId, previewRemoveIds, selectedSet, selectionMode, tessellation],
	);
	const selectedEdgeGeometry = reactHostPort.useMemo(
		() => selectionMode === "edge" ? buildEdgeOverlayGeometry(tessellation, selectedSet) : null,
		[selectedSet, selectionMode, tessellation],
	);
	const hoveredEdgeGeometry = reactHostPort.useMemo(
		() => selectionMode === "edge"
			? buildEdgeOverlayGeometry(
				tessellation,
				new Set([
					...(hoveredId != null && !selectedSet.has(hoveredId) ? [hoveredId] : []),
					...previewRemoveIds.filter((id) => !selectedSet.has(id)),
				]),
			)
			: null,
		[hoveredId, previewRemoveIds, selectedSet, selectionMode, tessellation],
	);
	const selectedVertexGeometry = reactHostPort.useMemo(
		() => selectionMode === "vertex" ? buildVertexOverlayGeometry(tessellation, selectedSet) : null,
		[selectedSet, selectionMode, tessellation],
	);
	const vertexPickGeometry = reactHostPort.useMemo(
		() => selectionMode === "vertex" ? buildVertexPickGeometry(tessellation) : null,
		[selectionMode, tessellation],
	);
	const vertexPickIds = reactHostPort.useMemo(() => {
		if (selectionMode !== "vertex") return [] as number[];
		const ids: number[] = [];
		const emitted = new Set<number>();
		for (let index = 0; index < tessellation.vertexIds.length; index += 1) {
			const id = tessellation.vertexIds[index]!;
			if (emitted.has(id)) continue;
			emitted.add(id);
			ids.push(id);
		}
		return ids;
	}, [selectionMode, tessellation]);
	const hoveredVertexGeometry = reactHostPort.useMemo(
		() => selectionMode === "vertex"
			? buildVertexOverlayGeometry(
				tessellation,
				new Set([
					...(hoveredId != null && !selectedSet.has(hoveredId) ? [hoveredId] : []),
					...previewRemoveIds.filter((id) => !selectedSet.has(id)),
				]),
			)
			: null,
		[hoveredId, previewRemoveIds, selectedSet, selectionMode, tessellation],
	);
	if (!surface) return null;

	const handlePick = (id: number, event: { stopPropagation: () => void; shiftKey?: boolean; ctrlKey?: boolean; metaKey?: boolean }) => {
		if (!pickEnabled) return;
		event.stopPropagation();
		const mode = marqueeModeFromModifiers(event);
		onPick(object.index, id, mode === "default" ? "invertive" : mode);
	};

	const handleHover = (id: number, event: { stopPropagation?: () => void }) => {
		if (!pickEnabled || onPaintAt) return;
		event.stopPropagation?.();
		onHover({ objectId: object.id, objectIndex: object.index, mode: selectionMode, id });
	};

	const paintFromHit = (event: THREE.Event & { faceIndex?: number | null; point?: THREE.Vector3; uv?: THREE.Vector2 }) => {
		if (!onPaintAt) return;
		event.stopPropagation?.();
		let u = event.uv?.x;
		let v = event.uv?.y;
		if (u == null || v == null) {
			if (event.faceIndex == null || !tessellation.indices.length) return;
			const i0 = tessellation.indices[event.faceIndex * 3] ?? 0;
			const i1 = tessellation.indices[event.faceIndex * 3 + 1] ?? 0;
			const i2 = tessellation.indices[event.faceIndex * 3 + 2] ?? 0;
			if (tessellation.uvs.length < 6) return;
			u = (tessellation.uvs[i0 * 2]! + tessellation.uvs[i1 * 2]! + tessellation.uvs[i2 * 2]!) / 3;
			v = (tessellation.uvs[i0 * 2 + 1]! + tessellation.uvs[i1 * 2 + 1]! + tessellation.uvs[i2 * 2 + 1]!) / 3;
		}
		onPaintAt(object.id, u, v);
	};

	return (
		<WorldLayer id={`lowpoly-mesh-${object.id}`} label={object.name}>
			<mesh
				geometry={surface}
				onClick={(event) => {
					if (onPaintAt) {
						paintFromHit(event);
						return;
					}
					if (selectionMode === "face" && event.faceIndex != null && tessellation.faceIds.length > event.faceIndex) {
						handlePick(tessellation.faceIds[event.faceIndex]!, event);
					} else if (selectionMode === "object") {
						handlePick(object.index, event);
					}
				}}
				onPointerMove={(event) => {
					if (onPaintAt) {
						if ((event.buttons & 1) !== 0) paintFromHit(event);
						return;
					}
					if (selectionMode === "face" && event.faceIndex != null) {
						const faceId = tessellation.faceIds[event.faceIndex];
						if (faceId != null) handleHover(faceId, event);
					} else if (selectionMode === "object") {
						handleHover(object.index, event);
					}
				}}
				onPointerOut={() => onHover(null)}
				onPointerDown={(event) => {
					if (!onPaintAt || event.button !== 0) return;
					paintFromHit(event);
				}}
			>
				<meshStandardMaterial
					color={isObjectSelected ? selectColor : isObjectHovered ? hoverColor : meshColor}
					map={paintTexture ?? undefined}
					flatShading={!object.smoothShading}
					side={THREE.DoubleSide}
					metalness={0.05}
					roughness={0.85}
				/>
			</mesh>
			{selectedFaceGeometry ? (
				<mesh geometry={selectedFaceGeometry} raycast={() => null}>
					<meshBasicMaterial color={selectColor} transparent opacity={0.62} depthWrite={false} polygonOffset polygonOffsetFactor={-2} side={THREE.DoubleSide} />
				</mesh>
			) : null}
			{hoveredFaceGeometry ? (
				<mesh geometry={hoveredFaceGeometry} raycast={() => null}>
					<meshBasicMaterial color={hoverColor} transparent opacity={0.48} depthWrite={false} polygonOffset polygonOffsetFactor={-3} side={THREE.DoubleSide} />
				</mesh>
			) : null}
			{edges ? (
				<lineSegments
					geometry={edges}
					onClick={(event) => {
						if (!pickEnabled || selectionMode !== "edge") return;
						const idx = (event as unknown as { index?: number }).index;
						if (typeof idx !== "number") return;
						const edgeIndex = Math.floor(idx / 2);
						const edgeId = tessellation.edgeIds[edgeIndex];
						if (edgeId == null) return;
						handlePick(edgeId, event);
					}}
					onPointerMove={(event) => {
						if (!pickEnabled || selectionMode !== "edge") return;
						const index = (event as unknown as { index?: number }).index;
						const edgeId = typeof index === "number" ? tessellation.edgeIds[Math.floor(index / 2)] : undefined;
						if (edgeId != null) handleHover(edgeId, event);
					}}
					onPointerOut={() => onHover(null)}
				>
					<lineBasicMaterial color={edgeColor} linewidth={1} />
				</lineSegments>
			) : null}
			{selectedEdgeGeometry ? (
				<lineSegments geometry={selectedEdgeGeometry} raycast={() => null}>
					<lineBasicMaterial color={selectColor} linewidth={3} />
				</lineSegments>
			) : null}
			{hoveredEdgeGeometry ? (
				<lineSegments geometry={hoveredEdgeGeometry} raycast={() => null}>
					<lineBasicMaterial color={hoverColor} linewidth={3} />
				</lineSegments>
			) : null}
			{selectionMode === "vertex" && vertexPickGeometry ? (
				<points
					geometry={vertexPickGeometry}
					onClick={(event) => {
						if (!pickEnabled) return;
						const idx = (event as unknown as { index?: number }).index;
						if (typeof idx !== "number") return;
						const vertexId = vertexPickIds[idx];
						if (vertexId == null) return;
						handlePick(vertexId, event);
					}}
					onPointerMove={(event) => {
						if (!pickEnabled) return;
						const index = (event as unknown as { index?: number }).index;
						const vertexId = typeof index === "number" ? vertexPickIds[index] : undefined;
						if (vertexId != null) handleHover(vertexId, event);
					}}
					onPointerOut={() => onHover(null)}
				>
					<pointsMaterial color={edgeColor} size={5} sizeAttenuation={false} />
				</points>
			) : null}
			{selectedVertexGeometry ? (
				<points geometry={selectedVertexGeometry} raycast={() => null}>
					<pointsMaterial color={selectColor} size={9} sizeAttenuation={false} depthTest={false} />
				</points>
			) : null}
			{hoveredVertexGeometry ? (
				<points geometry={hoveredVertexGeometry} raycast={() => null}>
					<pointsMaterial color={hoverColor} size={9} sizeAttenuation={false} depthTest={false} />
				</points>
			) : null}
		</WorldLayer>
	);
}

function LowpolyGumballLayer({
	active,
	target,
	onDragEnd,
	onDraggingChanged,
}: {
	readonly active: boolean;
	readonly target: THREE.Object3D | null;
	readonly onDragEnd: (kind: GumballHandleKind, before: GumballPose, after: GumballPose) => void;
	readonly onDraggingChanged: (active: boolean) => void;
}): React.ReactElement | null {
	if (!active || !target) return null;
	return (
		<UnifiedGumball
			target={target}
			onDragEnd={onDragEnd}
			onDraggingChanged={(dragging) => {
				gumballPointerConsumesCanvasEventRef.current = dragging;
				onDraggingChanged(dragging);
			}}
		/>
	);
}

function LowpolyMarqueeBridge({
	containerRef,
	sceneObjects,
	selectionMode,
	selectedIds,
	cameraRef,
	sizeRef,
	onCommit,
	onMarqueeOverlay,
	onLivePreview,
}: {
	readonly containerRef: React.RefObject<HTMLDivElement | null>;
	readonly sceneObjects: readonly LowpolySceneObject[];
	readonly selectionMode: LowpolySelectionMode;
	readonly selectedIds: readonly number[];
	readonly cameraRef: React.RefObject<THREE.Camera | null>;
	readonly sizeRef: React.RefObject<{ width: number; height: number }>;
	readonly onCommit: (ids: readonly number[], mode: SelectionMergeMode) => void;
	readonly onMarqueeOverlay: (overlay: { coverage: SelectionMarqueeCoverage; rect: { x: number; y: number; width: number; height: number } } | null) => void;
	readonly onLivePreview: (snapshot: { addIds: readonly number[]; removeIds: readonly number[] }) => void;
}): null {
	const gl = sceneHostPort.fiber.useThree((state) => state.gl);
	const invalidate = sceneHostPort.fiber.useThree((state) => state.invalidate);
	const marqueeRef = reactHostPort.useRef<{ tracking: boolean; active: boolean; start: { x: number; y: number }; initial: number[] }>({
		tracking: false,
		active: false,
		start: { x: 0, y: 0 },
		initial: [],
	});
	const sceneObjectsRef = reactHostPort.useRef(sceneObjects);
	const selectedIdsRef = reactHostPort.useRef(selectedIds);
	sceneObjectsRef.current = sceneObjects;
	selectedIdsRef.current = selectedIds;

	const clientToLocal = reactHostPort.useCallback((clientX: number, clientY: number) => {
		const host = containerRef.current;
		if (!host) return { x: clientX, y: clientY };
		const rect = host.getBoundingClientRect();
		return { x: clientX - rect.left, y: clientY - rect.top };
	}, [containerRef]);

	const resolveHits = reactHostPort.useCallback((start: { x: number; y: number }, end: { x: number; y: number }, crossing: boolean) => {
		const camera = cameraRef.current;
		const size = sizeRef.current;
		if (!camera) return [] as number[];
		const rect = {
			left: Math.min(start.x, end.x),
			top: Math.min(start.y, end.y),
			right: Math.max(start.x, end.x),
			bottom: Math.max(start.y, end.y),
		};
		const hits: number[] = [];
		for (const object of sceneObjectsRef.current) {
			const groupMatrix = new THREE.Matrix4();
			const pos = new THREE.Vector3(...object.transform.position);
			const quat = eulerToQuaternion(object.transform.rotation);
			const scale = new THREE.Vector3(...object.transform.scale);
			groupMatrix.compose(pos, quat, scale);
			if (selectionMode === "object") {
				const bounds = boundsFromPositions(object.tessellation.positions);
				if (!bounds) continue;
				const corners = [
					bounds.min.clone(),
					bounds.max.clone(),
					new THREE.Vector3(bounds.min.x, bounds.max.y, bounds.min.z),
					new THREE.Vector3(bounds.max.x, bounds.min.y, bounds.max.z),
				];
				let inside = false;
				for (const corner of corners) {
					corner.applyMatrix4(groupMatrix);
					const screen = projectWorldPoint(camera, size, corner);
					if (screen && screenRectContainsPoint(rect, screen, crossing)) {
						inside = true;
						break;
					}
				}
				if (inside) hits.push(object.index);
				continue;
			}
			if (!object.active) continue;
			const tess = object.tessellation;
			if (selectionMode === "vertex") {
				for (let i = 0; i < tess.positions.length; i += 3) {
					const point = new THREE.Vector3(tess.positions[i]!, tess.positions[i + 1]!, tess.positions[i + 2]!).applyMatrix4(groupMatrix);
					const screen = projectWorldPoint(camera, size, point);
					const vid = tess.vertexIds[i / 3];
					if (screen && vid != null && screenRectContainsPoint(rect, screen, crossing)) hits.push(vid);
				}
			} else if (selectionMode === "face") {
				for (let tri = 0; tri < tess.indices.length; tri += 3) {
					const i0 = tess.indices[tri]!;
					const i1 = tess.indices[tri + 1]!;
					const i2 = tess.indices[tri + 2]!;
					const centroid = new THREE.Vector3(
						(tess.positions[i0 * 3]! + tess.positions[i1 * 3]! + tess.positions[i2 * 3]!) / 3,
						(tess.positions[i0 * 3 + 1]! + tess.positions[i1 * 3 + 1]! + tess.positions[i2 * 3 + 1]!) / 3,
						(tess.positions[i0 * 3 + 2]! + tess.positions[i1 * 3 + 2]! + tess.positions[i2 * 3 + 2]!) / 3,
					).applyMatrix4(groupMatrix);
					const screen = projectWorldPoint(camera, size, centroid);
					const faceId = tess.faceIds[tri / 3];
					if (screen && faceId != null && screenRectContainsPoint(rect, screen, crossing)) hits.push(faceId);
				}
			} else if (selectionMode === "edge") {
				for (let e = 0; e < tess.edgePositions.length; e += 6) {
					const midpoint = new THREE.Vector3(
						(tess.edgePositions[e]! + tess.edgePositions[e + 3]!) / 2,
						(tess.edgePositions[e + 1]! + tess.edgePositions[e + 4]!) / 2,
						(tess.edgePositions[e + 2]! + tess.edgePositions[e + 5]!) / 2,
					).applyMatrix4(groupMatrix);
					const screen = projectWorldPoint(camera, size, midpoint);
					const edgeId = tess.edgeIds[e / 6];
					if (screen && edgeId != null && screenRectContainsPoint(rect, screen, crossing)) hits.push(edgeId);
				}
			}
		}
		return [...new Set(hits)];
	}, [cameraRef, selectionMode, sizeRef]);

	reactHostPort.useEffect(() => {
		const canvas = gl.domElement;
		const onPointerDown = (event: PointerEvent) => {
			if (event.button !== 0 || gumballPointerConsumesCanvasEventRef.current) return;
			const point = clientToLocal(event.clientX, event.clientY);
			marqueeRef.current = { tracking: true, active: false, start: point, initial: [...selectedIdsRef.current] };
		};
		const onPointerMove = (event: PointerEvent) => {
			if (!marqueeRef.current.tracking || gumballPointerConsumesCanvasEventRef.current) return;
			const point = clientToLocal(event.clientX, event.clientY);
			const start = marqueeRef.current.start;
			if (!marqueeRef.current.active && Math.hypot(point.x - start.x, point.y - start.y) < LOWPOLY_MARQUEE_THRESHOLD_PX) return;
			marqueeRef.current.active = true;
			const coverage = marqueeCoverageFromGesture({ method: "rectangle", startX: start.x, endX: point.x, path: [start, point] });
			const crossing = coverage === "partial";
			onMarqueeOverlay({
				coverage,
				rect: { x: Math.min(start.x, point.x), y: Math.min(start.y, point.y), width: Math.abs(point.x - start.x), height: Math.abs(point.y - start.y) },
			});
			const mode = marqueeModeFromModifiers(event);
			const hits = resolveHits(start, point, crossing);
			const merged = selectionMergeIds(mode, marqueeRef.current.initial, hits);
			const removed = marqueeRef.current.initial.filter((id) => !merged.includes(id));
			onLivePreview({
				addIds: merged.filter((id) => !marqueeRef.current.initial.includes(id)),
				removeIds: removed,
			});
			invalidate();
		};
		const onPointerUp = (event: PointerEvent) => {
			if (!marqueeRef.current.tracking) return;
			const point = clientToLocal(event.clientX, event.clientY);
			const start = marqueeRef.current.start;
			if (marqueeRef.current.active && Math.hypot(point.x - start.x, point.y - start.y) >= LOWPOLY_MARQUEE_THRESHOLD_PX) {
				const coverage = marqueeCoverageFromGesture({ method: "rectangle", startX: start.x, endX: point.x, path: [start, point] });
				const hits = resolveHits(start, point, coverage === "partial");
				const mode = marqueeModeFromModifiers(event);
				onCommit(hits, mode);
			}
			marqueeRef.current = { tracking: false, active: false, start: { x: 0, y: 0 }, initial: [] };
			onMarqueeOverlay(null);
			onLivePreview({ addIds: [], removeIds: [] });
		};
		canvas.addEventListener("pointerdown", onPointerDown, { capture: true });
		window.addEventListener("pointermove", onPointerMove);
		window.addEventListener("pointerup", onPointerUp);
		return () => {
			canvas.removeEventListener("pointerdown", onPointerDown, { capture: true } as AddEventListenerOptions);
			window.removeEventListener("pointermove", onPointerMove);
			window.removeEventListener("pointerup", onPointerUp);
		};
	}, [clientToLocal, gl.domElement, invalidate, onCommit, onLivePreview, onMarqueeOverlay, resolveHits]);

	return null;
}

export function LowpolyCanvas(props: LowpolyCanvasProps): React.ReactElement {
	const containerRef = reactHostPort.useRef<HTMLDivElement>(null);
	const cameraRef = reactHostPort.useRef<THREE.Camera | null>(null);
	const sizeRef = reactHostPort.useRef({ width: 1, height: 1 });
	const lodRef = reactHostPort.useRef(DEFAULT_MANUAL_LOD);
	const [projection, setProjection] = reactHostPort.useState<OrbitCameraProjection>("perspective");
	const [cameraState, setCameraState] = reactHostPort.useState<WorldCameraState>({
		position: [2.5, 2.0, 2.5],
		target: [0, 0, 0],
		zoom: 1,
	});
	const [canvasBackground, setCanvasBackground] = reactHostPort.useState(() => resolveSemanticColorHex("--canvas", "light-8-9"));
	const [meshColor, setMeshColor] = reactHostPort.useState(() => resolveSemanticColorHex("--panel"));
	const [edgeColor, setEdgeColor] = reactHostPort.useState(() => resolveSemanticColorHex("--border-normal-color"));
	const [selectColor, setSelectColor] = reactHostPort.useState(() => resolveSemanticColorHex("--active-base"));
	const [hoverColor, setHoverColor] = reactHostPort.useState(() => resolveSemanticColorHex("--hover-base"));
	const gumballTargetRef = reactHostPort.useRef<THREE.Object3D>(new THREE.Object3D());
	const [marquee, setMarquee] = reactHostPort.useState<{ coverage: SelectionMarqueeCoverage; rect: { x: number; y: number; width: number; height: number } } | null>(null);
	const [previewSelection, setPreviewSelection] = reactHostPort.useState<{ addIds: readonly number[]; removeIds: readonly number[] }>({ addIds: [], removeIds: [] });
	const [gumballDragActive, setGumballDragActive] = reactHostPort.useState(false);
	const paintTexturesRef = reactHostPort.useRef<Map<string, THREE.DataTexture>>(new Map());
	const strokeActiveRef = reactHostPort.useRef(false);
	const sessionEmittedFixtureRef = reactHostPort.useRef<string | null>(null);
	const [paintTextureTick, setPaintTextureTick] = reactHostPort.useState(0);
	const interactionMode = props.interactionMode ?? "model";
	const paintTool = props.paintTool ?? "brush";
	const paintColor = props.paintColor ?? [255, 64, 64, 255];
	const paintLayerIndex = props.paintLayerIndex ?? 0;
	const paintBrushSize = props.paintBrushSize ?? 16;
	const paintBrushOpacity = props.paintBrushOpacity ?? 1;
	const paintBrushHardness = props.paintBrushHardness ?? 0.5;

	const refreshPaintTexture = reactHostPort.useCallback(
		(objectId: string) => {
			if (!props.session) return;
			const pixels = props.session.compositePaintTexture(objectId);
			let texture = paintTexturesRef.current.get(objectId);
			if (!texture) {
				texture = new THREE.DataTexture(pixels, 1024, 1024, THREE.RGBAFormat);
				texture.flipY = false;
				texture.needsUpdate = true;
				paintTexturesRef.current.set(objectId, texture);
			} else {
				texture.image.data.set(pixels);
				texture.needsUpdate = true;
			}
			setPaintTextureTick((tick) => tick + 1);
			props.onPaintTextureRefresh?.();
		},
		[props.onPaintTextureRefresh, props.session],
	);

	reactHostPort.useEffect(() => {
		if (typeof document === "undefined") return;
		const sync = () => {
			clearColorResolveCache();
			setCanvasBackground(resolveSemanticColorHex("--canvas", "light-8-9"));
			setMeshColor(resolveSemanticColorHex("--panel"));
			setEdgeColor(resolveSemanticColorHex("--border-normal-color"));
			setSelectColor(resolveSemanticColorHex("--active-base"));
			setHoverColor(resolveSemanticColorHex("--hover-base"));
		};
		sync();
		const observer = new MutationObserver(sync);
		observer.observe(document.documentElement, {
			attributes: true,
			attributeFilter: ["class", "style", "data-theme", "data-ui-theme"],
		});
		return () => observer.disconnect();
	}, []);

	const refreshScene = reactHostPort.useCallback(() => {
		if (!props.session) return;
		const objects = tessellateAllLowpolySession(props.session);
		if (objects.length) props.onSceneChange?.(objects);
	}, [props.session, props.onSceneChange]);

	const emitFixtureChange = reactHostPort.useCallback(() => {
		if (!props.session) return;
		const json = props.session.fixtureJson();
		sessionEmittedFixtureRef.current = json;
		props.onFixtureChange?.(json);
	}, [props.onFixtureChange, props.session]);

	reactHostPort.useEffect(() => {
		if (!props.session || !isLowpolyFixtureReady(props.fixtureJson)) return;
		if (props.fixtureJson === sessionEmittedFixtureRef.current) {
			sessionEmittedFixtureRef.current = null;
			return;
		}
		if (!safeLoadLowpolyFixture(props.session, props.fixtureJson)) return;
		refreshScene();
	}, [props.fixtureJson, props.session, refreshScene]);

	const activeObject = props.sceneObjects.find((object) => object.active) ?? props.sceneObjects[0] ?? null;

	reactHostPort.useEffect(() => {
		if (interactionMode !== "paint" || !props.session) return;
		for (const object of props.sceneObjects) {
			refreshPaintTexture(object.id);
		}
	}, [interactionMode, props.paintTextureRevision, props.sceneObjects, props.session, refreshPaintTexture]);

	const gumballCentroid = reactHostPort.useMemo(() => {
		if (props.selectionMode === "object") {
			const selected = props.sceneObjects.filter((object) => props.selectedIds.includes(object.index));
			const targets = selected.length ? selected : activeObject ? [activeObject] : [];
			if (!targets.length) return [0, 0, 0] as [number, number, number];
			let x = 0;
			let y = 0;
			let z = 0;
			for (const object of targets) {
				const [cx, cy, cz] = meshCentroid(object.tessellation.positions);
				const [px, py, pz] = object.transform.position;
				x += cx + px;
				y += cy + py;
				z += cz + pz;
			}
			const count = targets.length;
			return [x / count, y / count, z / count] as [number, number, number];
		}
		if (!activeObject) return [0, 0, 0] as [number, number, number];
		const [x, y, z] = meshCentroid(activeObject.tessellation.positions);
		const [px, py, pz] = activeObject.transform.position;
		return [x + px, y + py, z + pz] as [number, number, number];
	}, [activeObject, props.sceneObjects, props.selectedIds, props.selectionMode]);

	reactHostPort.useEffect(() => {
		const [x, y, z] = gumballCentroid;
		gumballTargetRef.current.position.set(x, y, z);
		gumballTargetRef.current.updateMatrixWorld();
	}, [gumballCentroid]);

	const commitSelection = reactHostPort.useCallback(
		(ids: readonly number[], mode: SelectionMergeMode, objectIndex?: number, objectId?: string) => {
			const merged = selectionMergeIds(mode, [...props.selectedIds], [...ids]);
			const activeId =
				objectId ??
				(props.selectionMode === "object" && objectIndex != null
					? props.sceneObjects.find((object) => object.index === objectIndex)?.id
					: activeObject?.id);
			props.onSelectionChange?.(props.selectionMode, merged, activeId);
		},
		[activeObject?.id, props],
	);

	const onPick = reactHostPort.useCallback(
		(objectIndex: number, id: number, mode: SelectionMergeMode) => {
			const object = props.sceneObjects.find((entry) => entry.index === objectIndex);
			if (!object) return;
			commitSelection([id], mode, objectIndex, object.id);
		},
		[commitSelection, props],
	);

	const applyPaintAt = reactHostPort.useCallback(
		(objectId: string, u: number, v: number) => {
			if (!props.session || interactionMode !== "paint") return;
			if (!strokeActiveRef.current) {
				strokeActiveRef.current = true;
				props.onPaintStrokeBegin?.();
			}
			const [r, g, b, a] = paintColor;
			if (paintTool === "brush") {
				props.session.paintStroke(objectId, paintLayerIndex, u, v, paintBrushSize, r, g, b, a, paintBrushHardness, paintBrushOpacity, false);
			} else if (paintTool === "eraser") {
				props.session.paintStroke(objectId, paintLayerIndex, u, v, paintBrushSize, r, g, b, a, paintBrushHardness, paintBrushOpacity, true);
			} else if (paintTool === "fill") {
				props.session.fillBucket(objectId, paintLayerIndex, u, v, r, g, b, a);
				strokeActiveRef.current = false;
				props.onPaintStrokeEnd?.();
			} else if (paintTool === "eyedropper") {
				const sample = props.session.samplePixel(objectId, u, v);
				if (sample.length >= 4) {
					/* eyedropper samples composite texture only */
				}
				return;
			}
			refreshPaintTexture(objectId);
		},
		[interactionMode, paintBrushHardness, paintBrushOpacity, paintBrushSize, paintColor, paintLayerIndex, paintTool, props, refreshPaintTexture],
	);

	reactHostPort.useEffect(() => {
		const endStroke = () => {
			if (!strokeActiveRef.current) return;
			strokeActiveRef.current = false;
			props.onPaintStrokeEnd?.();
		};
		window.addEventListener("pointerup", endStroke);
		return () => window.removeEventListener("pointerup", endStroke);
	}, [props.onPaintStrokeEnd]);

	const onGumballDragEnd = reactHostPort.useCallback(
		(kind: GumballHandleKind, before: GumballPose, after: GumballPose) => {
			if (!props.session || interactionMode !== "model") return;
			try {
				const dx = after.position[0] - before.position[0];
				const dy = after.position[1] - before.position[1];
				const dz = after.position[2] - before.position[2];
				if (props.transformTool === "move") {
					props.session.translateSelection(dx, dy, dz);
				} else if (props.transformTool === "rotate") {
					props.session.rotateSelection(0, 1, 0, after.rotation[1] - before.rotation[1]);
				} else {
					const sx = after.scale[0] / Math.max(before.scale[0], 1e-6);
					const sy = after.scale[1] / Math.max(before.scale[1], 1e-6);
					const sz = after.scale[2] / Math.max(before.scale[2], 1e-6);
					props.session.scaleSelection(sx, sy, sz);
				}
				emitFixtureChange();
				refreshScene();
			} catch {
				/* transform may fail without mesh selection */
			}
		},
		[interactionMode, emitFixtureChange, props, refreshScene],
	);

	const onProjectionChange = reactHostPort.useCallback((next: OrbitCameraProjection) => {
		setProjection(next);
		setCameraState((current) => applyOrbitProjectionToCameraState(current, next));
	}, []);

	const paintHandler = interactionMode === "paint" && activeObject ? applyPaintAt : undefined;
	const marqueeActive = marquee != null;

	return (
		<div
			ref={containerRef}
			className={cn("relative h-full min-h-0 w-full", canvasHostRootClass, props.className)}
			data-lowpoly-canvas=""
			data-lowpoly-hover-target={
				props.hoveredTarget
					? `${props.hoveredTarget.objectId}:${props.hoveredTarget.mode}:${props.hoveredTarget.id}`
					: undefined
			}
			data-lowpoly-selection={`${props.selectionMode}:${props.selectedIds.join(",")}`}
		>
			<WorldCanvas
				className="h-full w-full"
				frameloop={gumballDragActive ? "always" : "demand"}
				background={canvasBackground}
				overlay={<WorldOrbitProjectionSwitch projection={projection} onProjectionChange={onProjectionChange} />}
			>
				<WorldLodBridge
					lodRef={lodRef}
					distanceReference={8}
					gridFactor={DEFAULT_LOD_GRID_FACTOR}
					gridSnapEnabled
					showLodGrid
					automaticLod
					depthVariableLod={false}
					manualLod={DEFAULT_MANUAL_LOD}
					gridDatum={[0, 0, 0]}
				>
					<WorldOrbitViewSnapGateProvider>
						<WorldOrbitCameraViewRig state={cameraState} seedKey="lowpoly" perspectiveFov={45} onCamera={() => {}} />
						<WorldOrbitGated controlsKey="lowpoly" projection={projection} zoom={cameraState.zoom} onCamera={setCameraState} controlsGate={gumballDragActive} />
						<WorldOrbitViewControls onCameraChange={(next) => setCameraState(next)} />
						<WorldCameraInvalidator />
						<LowpolyLineRaycastThreshold />
						<LowpolyCameraBridge onCamera={(camera, size) => {
							cameraRef.current = camera;
							sizeRef.current = size;
						}} />
						<LowpolyMarqueeBridge
							containerRef={containerRef}
							sceneObjects={props.sceneObjects}
							selectionMode={props.selectionMode}
							selectedIds={props.selectedIds}
							cameraRef={cameraRef}
							sizeRef={sizeRef}
							onCommit={(ids, mode) => commitSelection(ids, mode)}
							onMarqueeOverlay={setMarquee}
							onLivePreview={setPreviewSelection}
						/>
						<LowpolySceneInvalidator token={props.sceneObjects.length} />
						<ambientLight intensity={0.45} />
						<directionalLight position={[12, 18, 10]} intensity={1.1} />
						<directionalLight position={[-10, -8, 6]} intensity={0.35} />
						{props.sceneObjects.map((object) => (
							<group key={object.id} position={object.transform.position} rotation={object.transform.rotation} scale={object.transform.scale}>
								<LowpolyMeshLayer
									object={object}
									selectedIds={props.selectedIds}
									previewAddIds={previewSelection.addIds}
									previewRemoveIds={previewSelection.removeIds}
									hoveredTarget={props.hoveredTarget ?? null}
									selectionMode={props.selectionMode}
									meshColor={meshColor}
									edgeColor={edgeColor}
									selectColor={selectColor}
									hoverColor={hoverColor}
									paintTexture={interactionMode === "paint" ? paintTexturesRef.current.get(object.id) ?? null : null}
									pickEnabled={!gumballDragActive && !marqueeActive}
									onPick={onPick}
									onHover={(target) => props.onHoverChange?.(target)}
									onPaintAt={object.active ? paintHandler : undefined}
								/>
							</group>
						))}
						<LowpolyGumballLayer
							active={interactionMode === "model" && (props.selectedIds.length > 0 || props.selectionMode === "object")}
							target={gumballTargetRef.current}
							onDragEnd={onGumballDragEnd}
							onDraggingChanged={setGumballDragActive}
						/>
					</WorldOrbitViewSnapGateProvider>
				</WorldLodBridge>
			</WorldCanvas>
			{marquee ? <SelectionMarquee coverage={marquee.coverage} shape="rect" rect={marquee.rect} /> : null}
		</div>
	);
}

//#endregion LowpolyCanvas

//#region LowpolyUvCanvas

export interface LowpolyUvCanvasProps {
	readonly sceneObject: LowpolySceneObject | null;
	readonly session: LowpolySessionWasm | null;
	readonly paintTool?: LowpolyPaintTool;
	readonly paintLayerIndex?: number;
	readonly paintColor?: readonly [number, number, number, number];
	readonly paintBrushSize?: number;
	readonly paintBrushOpacity?: number;
	readonly paintBrushHardness?: number;
	readonly paintTextureRevision?: number;
	readonly className?: string;
	readonly onFixtureChange?: (json: string) => void;
	readonly onPaintStrokeBegin?: () => void;
	readonly onPaintStrokeEnd?: () => void;
	readonly onPaintTextureRefresh?: () => void;
}

export function LowpolyUvCanvas(props: LowpolyUvCanvasProps): React.ReactElement {
	const canvasRef = reactHostPort.useRef<HTMLCanvasElement>(null);
	const paintTool = props.paintTool ?? "brush";
	const paintColor = props.paintColor ?? [255, 64, 64, 255];
	const paintLayerIndex = props.paintLayerIndex ?? 0;
	const paintBrushSize = props.paintBrushSize ?? 16;
	const paintBrushOpacity = props.paintBrushOpacity ?? 1;
	const paintBrushHardness = props.paintBrushHardness ?? 0.5;
	const [zoom, setZoom] = reactHostPort.useState(1);
	const [pan, setPan] = reactHostPort.useState({ x: 0, y: 0 });
	const dragRef = reactHostPort.useRef<{ painting: boolean; panning: boolean; lastX: number; lastY: number }>({ painting: false, panning: false, lastX: 0, lastY: 0 });

	const draw = reactHostPort.useCallback(() => {
		const canvas = canvasRef.current;
		const object = props.sceneObject;
		if (!canvas || !object || !props.session) return;
		const ctx = canvas.getContext("2d");
		if (!ctx) return;
		const width = canvas.width;
		const height = canvas.height;
		const checkerA = resolveSemanticColorHex("--panel");
		const checkerB = resolveSemanticColorHex("--hover-base");
		const edgeStroke = resolveSemanticColorHex("--border-normal-color");
		const seamStroke = resolveSemanticColorHex("--accent-secondary");
		const unitBorder = resolveSemanticColorHex("--active-base");
		ctx.setTransform(1, 0, 0, 1, 0, 0);
		ctx.clearRect(0, 0, width, height);
		ctx.save();
		ctx.translate(pan.x, pan.y);
		ctx.scale(zoom, zoom);
		const cell = Math.max(8, Math.round(width / 16));
		for (let row = 0; row < Math.ceil(height / cell); row += 1) {
			for (let col = 0; col < Math.ceil(width / cell); col += 1) {
				ctx.fillStyle = (row + col) % 2 === 0 ? checkerA : checkerB;
				ctx.fillRect(col * cell, row * cell, cell, cell);
			}
		}
		ctx.strokeStyle = edgeStroke;
		ctx.lineWidth = 1 / zoom;
		for (let x = 0; x <= width; x += cell) {
			ctx.beginPath();
			ctx.moveTo(x, 0);
			ctx.lineTo(x, height);
			ctx.stroke();
		}
		for (let y = 0; y <= height; y += cell) {
			ctx.beginPath();
			ctx.moveTo(0, y);
			ctx.lineTo(width, y);
			ctx.stroke();
		}
		const pixels = props.session.compositePaintTexture(object.id);
		const image = new ImageData(new Uint8ClampedArray(pixels), 1024, 1024);
		const offscreen = document.createElement("canvas");
		offscreen.width = 1024;
		offscreen.height = 1024;
		offscreen.getContext("2d")?.putImageData(image, 0, 0);
		ctx.drawImage(offscreen, 0, 0, width, height);
		const tess = object.tessellation;
		if (tess.edgeUvs.length >= 4 && tess.edgeIds.length > 0) {
			for (let edge = 0; edge < tess.edgeIds.length; edge += 1) {
				const u0 = tess.edgeUvs[edge * 4]! * width;
				const v0 = (1 - tess.edgeUvs[edge * 4 + 1]!) * height;
				const u1 = tess.edgeUvs[edge * 4 + 2]! * width;
				const v1 = (1 - tess.edgeUvs[edge * 4 + 3]!) * height;
				const isSeam = tess.edgeIsSeam[edge] === 1;
				ctx.strokeStyle = isSeam ? seamStroke : edgeStroke;
				ctx.setLineDash(isSeam ? [6 / zoom, 4 / zoom] : []);
				ctx.beginPath();
				ctx.moveTo(u0, v0);
				ctx.lineTo(u1, v1);
				ctx.stroke();
			}
		}
		ctx.setLineDash([]);
		ctx.strokeStyle = unitBorder;
		ctx.lineWidth = 2 / zoom;
		ctx.strokeRect(0, 0, width, height);
		ctx.restore();
	}, [pan.x, pan.y, props.sceneObject, props.session, zoom]);

	reactHostPort.useEffect(() => {
		draw();
	}, [draw, props.paintTextureRevision, props.sceneObject, props.session]);

	const paintAt = reactHostPort.useCallback(
		(clientX: number, clientY: number) => {
			const canvas = canvasRef.current;
			const object = props.sceneObject;
			if (!canvas || !object || !props.session) return;
			const rect = canvas.getBoundingClientRect();
			const x = ((clientX - rect.left - pan.x) / zoom / rect.width);
			const y = 1 - (clientY - rect.top - pan.y) / zoom / rect.height;
			const [r, g, b, a] = paintColor;
			if (paintTool === "brush") {
				props.session.paintStroke(object.id, paintLayerIndex, x, y, paintBrushSize, r, g, b, a, paintBrushHardness, paintBrushOpacity, false);
			} else if (paintTool === "eraser") {
				props.session.paintStroke(object.id, paintLayerIndex, x, y, paintBrushSize, r, g, b, a, paintBrushHardness, paintBrushOpacity, true);
			} else if (paintTool === "fill") {
				props.session.fillBucket(object.id, paintLayerIndex, x, y, r, g, b, a);
				props.onPaintStrokeEnd?.();
			}
			draw();
			props.onPaintTextureRefresh?.();
		},
		[draw, paintBrushHardness, paintBrushOpacity, paintBrushSize, paintColor, paintLayerIndex, paintTool, props.onPaintStrokeEnd, props.onPaintTextureRefresh, props.sceneObject, props.session, pan.x, pan.y, zoom],
	);

	reactHostPort.useEffect(() => {
		const canvas = canvasRef.current;
		if (!canvas) return;
		const onPointerDown = (event: PointerEvent) => {
			dragRef.current.lastX = event.clientX;
			dragRef.current.lastY = event.clientY;
			if (event.button === 1 || event.altKey) {
				dragRef.current.panning = true;
				return;
			}
			dragRef.current.painting = true;
			props.onPaintStrokeBegin?.();
			paintAt(event.clientX, event.clientY);
		};
		const onPointerMove = (event: PointerEvent) => {
			if (dragRef.current.panning) {
				setPan((current) => ({
					x: current.x + event.clientX - dragRef.current.lastX,
					y: current.y + event.clientY - dragRef.current.lastY,
				}));
			} else if (dragRef.current.painting) {
				paintAt(event.clientX, event.clientY);
			}
			dragRef.current.lastX = event.clientX;
			dragRef.current.lastY = event.clientY;
		};
		const onPointerUp = () => {
			if (dragRef.current.painting) props.onPaintStrokeEnd?.();
			dragRef.current.painting = false;
			dragRef.current.panning = false;
		};
		const onWheel = (event: WheelEvent) => {
			event.preventDefault();
			setZoom((current) => Math.max(0.25, Math.min(8, current * (event.deltaY > 0 ? 0.9 : 1.1))));
		};
		canvas.addEventListener("pointerdown", onPointerDown);
		window.addEventListener("pointermove", onPointerMove);
		window.addEventListener("pointerup", onPointerUp);
		canvas.addEventListener("wheel", onWheel, { passive: false });
		return () => {
			canvas.removeEventListener("pointerdown", onPointerDown);
			window.removeEventListener("pointermove", onPointerMove);
			window.removeEventListener("pointerup", onPointerUp);
			canvas.removeEventListener("wheel", onWheel);
		};
	}, [paintAt, props.onPaintStrokeBegin, props.onPaintStrokeEnd]);

	reactHostPort.useEffect(() => {
		const canvas = canvasRef.current;
		if (!canvas) return;
		const resize = () => {
			const parent = canvas.parentElement;
			if (!parent) return;
			canvas.width = parent.clientWidth;
			canvas.height = parent.clientHeight;
			draw();
		};
		resize();
		const observer = new ResizeObserver(resize);
		observer.observe(canvas.parentElement ?? canvas);
		return () => observer.disconnect();
	}, [draw]);

	return <canvas ref={canvasRef} className={cn("h-full w-full touch-none", props.className)} data-lowpoly-uv-canvas="" />;
}

//#endregion LowpolyUvCanvas

if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;
	describe("tessellationFromWasm", () => {
		it("wraps arrays", () => {
			const tess = tessellationFromWasm({
				positions: [0, 0, 0, 1, 0, 0],
				normals: [0, 1, 0, 0, 1, 0],
				indices: [0, 1, 2],
				edgePositions: [0, 0, 0, 1, 0, 0],
				faceIds: [0],
				vertexIds: [0, 1],
				edgeIds: [0],
				edgeUvs: [0, 0, 1, 0],
				edgeIsSeam: [1],
				uvs: [0, 0, 1, 0],
			});
			expect(tess.positions.length).toBe(6);
			expect(tess.faceIds.length).toBe(1);
			expect(tess.edgeUvs.length).toBe(4);
			expect(tess.edgeIsSeam[0]).toBe(1);
		});
	});
	describe("safeLoadLowpolyFixture", () => {
		it("rejects empty fixtures", () => {
			const session = {
				loadFixtureJson: () => {
					throw new Error("should not load");
				},
			} as LowpolySessionWasm;
			expect(safeLoadLowpolyFixture(session, '{"schema":"lowpoly.fixture","objects":[],"activeObjectId":"","selection":{"mode":"object","ids":[]}}')).toBe(false);
		});
	});
}
