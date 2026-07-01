// #region 🧲Header
/// <reference types="vite/client" />
/// <reference types="vitest/importMeta" />
/** @emoji 🔷 `@semio-tech/lowpoly-react` — low-poly mesh editor viewport. */
// #endregion 🧲Header

import type { LowpolySelectionModeV1, LowpolyTessellation } from "@semio-tech/lowpoly-core";
import {
	DEFAULT_LOD_GRID_FACTOR,
	DEFAULT_MANUAL_LOD,
	WorldCameraInvalidator,
	WorldCanvas,
	WorldLayer,
	WorldLodBridge,
	WorldOrbitCameraViewRig,
	WorldOrbitGated,
	WorldOrbitViewControls,
	WorldOrbitViewSnapGateProvider,
	type WorldCameraState,
} from "@semio-tech/infinite-world-r3f";
import {
	SelectionMarquee,
	UnifiedGumball,
	cn,
	gumballPointerConsumesCanvasEventRef,
	reactHostPort,
	sceneHostPort,
	type GumballHandleKind,
	type GumballPose,
	type SelectionMarqueeCoverage,
} from "@semio-tech/ui-react";

const THREE = sceneHostPort.three;

//#region WasmBridge

export type LowpolySessionWasm = {
	fixtureJson(): string;
	loadFixtureJson(json: string): void;
	addPrimitive(kind: string): string;
	setActiveObject(id: string): void;
	setSelection(mode: string, ids: number[]): void;
	tessellateActive(): {
		positions: number[];
		normals: number[];
		indices: number[];
		edgePositions: number[];
	};
	exportObjActive(): string;
	extrudeFaces(distance: number): void;
	insetFaces(amount: number): void;
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
};

let lowpolySessionPromise: Promise<typeof import("../core/pkg/lowpoly_core.js")> | null = null;

export async function ensureLowpolyWasm(): Promise<typeof import("../core/pkg/lowpoly_core.js")> {
	lowpolySessionPromise ??= import("../core/pkg/lowpoly_core.js");
	return lowpolySessionPromise;
}

export async function createLowpolySession(fixtureJson?: string): Promise<LowpolySessionWasm> {
	const wasm = await ensureLowpolyWasm();
	await wasm.default();
	return new wasm.LowpolySession(fixtureJson ?? undefined) as unknown as LowpolySessionWasm;
}

export function tessellationFromWasm(raw: {
	positions: number[];
	normals: number[];
	indices: number[];
	edgePositions: number[];
}): LowpolyTessellation {
	return {
		positions: new Float32Array(raw.positions),
		normals: new Float32Array(raw.normals),
		indices: new Uint32Array(raw.indices),
		edgePositions: new Float32Array(raw.edgePositions),
	};
}

//#endregion WasmBridge

//#region MeshBuffers

function buildMeshGeometry(tess: LowpolyTessellation): {
	surface: THREE.BufferGeometry | null;
	edges: THREE.BufferGeometry | null;
} {
	let surface: THREE.BufferGeometry | null = null;
	if (tess.positions.length > 0 && tess.indices.length > 0) {
		const geometry = new THREE.BufferGeometry();
		geometry.setAttribute("position", new THREE.Float32BufferAttribute(tess.positions, 3));
		geometry.setAttribute("normal", new THREE.Float32BufferAttribute(tess.normals, 3));
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

//#endregion MeshBuffers

//#region LowpolyCanvas

export type LowpolyTransformTool = "move" | "rotate" | "scale";

export interface LowpolyCanvasProps {
	readonly fixtureJson: string;
	readonly selectionMode: LowpolySelectionModeV1;
	readonly selectedIds: readonly number[];
	readonly transformTool: LowpolyTransformTool;
	readonly session: LowpolySessionWasm | null;
	readonly tessellation: LowpolyTessellation | null;
	readonly className?: string;
	readonly onFixtureChange?: (json: string) => void;
	readonly onSelectionChange?: (mode: LowpolySelectionModeV1, ids: readonly number[]) => void;
	readonly onTessellationChange?: (tess: LowpolyTessellation) => void;
}

function LowpolyMeshLayer({
	tessellation,
	selectedIds,
	selectionMode,
	onPick,
}: {
	readonly tessellation: LowpolyTessellation | null;
	readonly selectedIds: readonly number[];
	readonly selectionMode: LowpolySelectionModeV1;
	readonly onPick: (id: number) => void;
}): React.ReactElement | null {
	const { surface, edges } = reactHostPort.useMemo(
		() => (tessellation ? buildMeshGeometry(tessellation) : { surface: null, edges: null }),
		[tessellation],
	);
	if (!surface) return null;
	return (
		<WorldLayer id="lowpoly-mesh" label="Lowpoly Mesh">
			<mesh
				geometry={surface}
				onClick={(event) => {
					event.stopPropagation();
					if (selectionMode === "face" && event.faceIndex != null) {
						onPick(event.faceIndex);
					} else if (selectionMode === "object") {
						onPick(0);
					}
				}}
			>
				<meshStandardMaterial color="#7eb8da" flatShading side={THREE.DoubleSide} />
			</mesh>
			{edges ? (
				<lineSegments geometry={edges}>
					<lineBasicMaterial color="#1a2a3a" linewidth={1} />
				</lineSegments>
			) : null}
			{selectionMode === "vertex" && tessellation ? (
				<points
					onClick={(event) => {
						event.stopPropagation();
						const idx = (event as unknown as { index?: number }).index;
						if (typeof idx === "number") onPick(idx);
					}}
				>
					<bufferGeometry>
						<bufferAttribute attach="attributes-position" args={[tessellation.positions, 3]} />
					</bufferGeometry>
					<pointsMaterial color="#ffcc00" size={6} sizeAttenuation={false} />
				</points>
			) : null}
		</WorldLayer>
	);
}

function LowpolyGumballLayer({
	active,
	target,
	onDragEnd,
}: {
	readonly active: boolean;
	readonly target: THREE.Object3D | null;
	readonly onDragEnd: (kind: GumballHandleKind, before: GumballPose, after: GumballPose) => void;
}): React.ReactElement | null {
	if (!active || !target) return null;
	return (
		<UnifiedGumball
			target={target}
			onDragEnd={onDragEnd}
			onDraggingChanged={(dragging) => {
				gumballPointerConsumesCanvasEventRef.current = dragging;
			}}
		/>
	);
}

export function LowpolyCanvas(props: LowpolyCanvasProps): React.ReactElement {
	const lodRef = reactHostPort.useRef(DEFAULT_MANUAL_LOD);
	const [cameraState, setCameraState] = reactHostPort.useState<WorldCameraState>({
		position: [2.5, 2.0, 2.5],
		target: [0, 0, 0],
		zoom: 1,
	});
	const gumballTargetRef = reactHostPort.useRef<THREE.Object3D>(new THREE.Object3D());
	const [marquee, setMarquee] = reactHostPort.useState<SelectionMarqueeCoverage | null>(null);

	const refreshTessellation = reactHostPort.useCallback(() => {
		if (!props.session) return;
		const raw = props.session.tessellateActive();
		const tess = tessellationFromWasm(raw);
		props.onTessellationChange?.(tess);
	}, [props.session, props.onTessellationChange]);

	reactHostPort.useEffect(() => {
		if (!props.session) return;
		props.session.loadFixtureJson(props.fixtureJson);
		refreshTessellation();
	}, [props.fixtureJson, props.session, refreshTessellation]);

	const onPick = reactHostPort.useCallback(
		(id: number) => {
			props.onSelectionChange?.(props.selectionMode, [id]);
		},
		[props.onSelectionChange, props.selectionMode],
	);

	const onGumballDragEnd = reactHostPort.useCallback(
		(kind: GumballHandleKind, before: GumballPose, after: GumballPose) => {
			if (!props.session) return;
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
			props.onFixtureChange?.(props.session.fixtureJson());
			refreshTessellation();
		},
		[props.session, props.transformTool, props.onFixtureChange, refreshTessellation],
	);

	return (
		<div className={cn("relative h-full min-h-0 w-full", props.className)} data-lowpoly-canvas="">
			<WorldCanvas className="absolute inset-0" frameloop="demand">
				<WorldLodBridge
					lodRef={lodRef}
					distanceReference={8}
					gridFactor={DEFAULT_LOD_GRID_FACTOR}
					gridSnapEnabled
					showLodGrid={false}
					automaticLod
					depthVariableLod={false}
					manualLod={DEFAULT_MANUAL_LOD}
				>
					<WorldOrbitViewSnapGateProvider>
						<WorldOrbitCameraViewRig state={cameraState} seedKey="lowpoly" onCamera={() => {}} />
						<WorldOrbitGated
							camera={null}
							onCamera={setCameraState}
							controlsGate={!gumballPointerConsumesCanvasEventRef.current}
						/>
						<WorldOrbitViewControls />
						<WorldCameraInvalidator />
						<LowpolyMeshLayer
							tessellation={props.tessellation}
							selectedIds={props.selectedIds}
							selectionMode={props.selectionMode}
							onPick={onPick}
						/>
						<LowpolyGumballLayer
							active={props.transformTool !== undefined}
							target={gumballTargetRef.current}
							onDragEnd={onGumballDragEnd}
						/>
					</WorldOrbitViewSnapGateProvider>
				</WorldLodBridge>
			</WorldCanvas>
			{marquee ? <SelectionMarquee coverage={marquee} shape="rect" rect={{ x: 0, y: 0, width: 0, height: 0 }} /> : null}
		</div>
	);
}

//#endregion LowpolyCanvas

if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;
	describe("tessellationFromWasm", () => {
		it("wraps arrays", () => {
			const tess = tessellationFromWasm({
				positions: [0, 0, 0, 1, 0, 0],
				normals: [0, 1, 0, 0, 1, 0],
				indices: [0, 1, 2],
				edgePositions: [0, 0, 0, 1, 0, 0],
			});
			expect(tess.positions.length).toBe(6);
			expect(tess.indices.length).toBe(3);
		});
	});
}
