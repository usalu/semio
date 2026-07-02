// #region 🧲Header
/// <reference types="vite/client" />
/// <reference types="vitest/importMeta" />
/** @emoji 🔷 `@semio-tech/lowpoly-react` — low-poly mesh editor viewport. */
// #endregion 🧲Header

import type { LowpolySelectionMode, LowpolyTessellation } from "@semio-tech/lowpoly-core";
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
	reactHostPort,
	sceneHostPort,
	type GumballHandleKind,
	type GumballPose,
	type SelectionMarqueeCoverage,
} from "@semio-tech/ui-react";
import { clearColorResolveCache, resolveSemanticColorHex } from "@semio-tech/ui-styling";

const THREE = sceneHostPort.three;

//#region WasmBridge

export type LowpolySessionWasm = {
	fixtureJson(): string;
	loadFixtureJson(json: string): void;
	addPrimitive(kind: string): string;
	setActiveObject(id: string): void;
	setSelection(mode: string, ids: number[]): void;
	tessellateActive(): string;
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

export function parseLowpolyTessellationJson(json: string): LowpolyTessellation | null {
	try {
		const raw = JSON.parse(json) as {
			positions?: number[];
			normals?: number[];
			indices?: number[];
			edgePositions?: number[];
		};
		if (!raw.positions?.length || !raw.indices?.length) return null;
		return tessellationFromWasm({
			positions: raw.positions,
			normals: raw.normals ?? [],
			indices: raw.indices,
			edgePositions: raw.edgePositions ?? [],
		});
	} catch {
		return null;
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
	readonly selectionMode: LowpolySelectionMode;
	readonly selectedIds: readonly number[];
	readonly transformTool: LowpolyTransformTool;
	readonly session: LowpolySessionWasm | null;
	readonly tessellation: LowpolyTessellation | null;
	readonly className?: string;
	readonly onFixtureChange?: (json: string) => void;
	readonly onSelectionChange?: (mode: LowpolySelectionMode, ids: readonly number[]) => void;
	readonly onTessellationChange?: (tess: LowpolyTessellation) => void;
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

function LowpolySceneInvalidator({ token }: { readonly token: number }): null {
	const invalidate = sceneHostPort.fiber.useThree((state) => state.invalidate);
	reactHostPort.useEffect(() => {
		invalidate();
	}, [invalidate, token]);
	return null;
}

function LowpolyMeshLayer({
	tessellation,
	selectedIds,
	selectionMode,
	meshColor,
	edgeColor,
	onPick,
}: {
	readonly tessellation: LowpolyTessellation | null;
	readonly selectedIds: readonly number[];
	readonly selectionMode: LowpolySelectionMode;
	readonly meshColor: string;
	readonly edgeColor: string;
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
				<meshStandardMaterial color={meshColor} flatShading side={THREE.DoubleSide} metalness={0.05} roughness={0.85} />
			</mesh>
			{edges ? (
				<lineSegments geometry={edges}>
					<lineBasicMaterial color={edgeColor} linewidth={1} />
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

export function LowpolyCanvas(props: LowpolyCanvasProps): React.ReactElement {
	const containerRef = reactHostPort.useRef<HTMLDivElement>(null);
	const lodRef = reactHostPort.useRef(DEFAULT_MANUAL_LOD);
	const [projection, setProjection] = reactHostPort.useState<OrbitCameraProjection>("perspective");
	const [cameraState, setCameraState] = reactHostPort.useState<WorldCameraState>({
		position: [2.5, 2.0, 2.5],
		target: [0, 0, 0],
		zoom: 1,
	});
	const [canvasBackground, setCanvasBackground] = reactHostPort.useState(() => resolveSemanticColorHex("--canvas", "light-8-9"));
	const [meshColor, setMeshColor] = reactHostPort.useState(() => resolveSemanticColorHex("--accent-8"));
	const [edgeColor, setEdgeColor] = reactHostPort.useState(() => resolveSemanticColorHex("--dark-4"));
	const gumballTargetRef = reactHostPort.useRef<THREE.Object3D>(new THREE.Object3D());
	const [marquee, setMarquee] = reactHostPort.useState<SelectionMarqueeCoverage | null>(null);
	const [gumballDragActive, setGumballDragActive] = reactHostPort.useState(false);

	reactHostPort.useEffect(() => {
		if (typeof document === "undefined") return;
		const sync = () => {
			clearColorResolveCache();
			setCanvasBackground(resolveSemanticColorHex("--canvas", "light-8-9"));
			setMeshColor(resolveSemanticColorHex("--accent-8"));
			setEdgeColor(resolveSemanticColorHex("--dark-4"));
		};
		sync();
		const observer = new MutationObserver(sync);
		observer.observe(document.documentElement, {
			attributes: true,
			attributeFilter: ["class", "style", "data-theme", "data-ui-theme"],
		});
		return () => observer.disconnect();
	}, []);

	reactHostPort.useEffect(() => {
		if (!props.tessellation?.positions.length) return;
		const [x, y, z] = meshCentroid(props.tessellation.positions);
		gumballTargetRef.current.position.set(x, y, z);
		gumballTargetRef.current.updateMatrixWorld();
	}, [props.tessellation]);

	const refreshTessellation = reactHostPort.useCallback(() => {
		if (!props.session) return;
		const tess = tessellateLowpolySession(props.session);
		if (tess) props.onTessellationChange?.(tess);
	}, [props.session, props.onTessellationChange]);

	reactHostPort.useEffect(() => {
		if (!props.session || !isLowpolyFixtureReady(props.fixtureJson)) return;
		if (!safeLoadLowpolyFixture(props.session, props.fixtureJson)) return;
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
				props.onFixtureChange?.(props.session.fixtureJson());
				refreshTessellation();
			} catch {
				/* transform may fail without mesh selection */
			}
		},
		[props.session, props.transformTool, props.onFixtureChange, refreshTessellation],
	);

	const onProjectionChange = reactHostPort.useCallback((next: OrbitCameraProjection) => {
		setProjection(next);
		setCameraState((current) => applyOrbitProjectionToCameraState(current, next));
	}, []);

	return (
		<div
			ref={containerRef}
			className={cn("relative h-full min-h-0 w-full", canvasHostRootClass, props.className)}
			data-lowpoly-canvas=""
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
						<WorldOrbitGated
							controlsKey="lowpoly"
							projection={projection}
							zoom={cameraState.zoom}
							onCamera={setCameraState}
							controlsGate={!gumballPointerConsumesCanvasEventRef.current}
						/>
						<WorldOrbitViewControls />
						<WorldCameraInvalidator />
						<LowpolySceneInvalidator token={props.tessellation?.positions.length ?? 0} />
						<ambientLight intensity={0.45} />
						<directionalLight position={[12, 18, 10]} intensity={1.1} />
						<directionalLight position={[-10, -8, 6]} intensity={0.35} />
						<LowpolyMeshLayer
							tessellation={props.tessellation}
							selectedIds={props.selectedIds}
							selectionMode={props.selectionMode}
							meshColor={meshColor}
							edgeColor={edgeColor}
							onPick={onPick}
						/>
						<LowpolyGumballLayer
							active={props.selectedIds.length > 0 || props.selectionMode === "object"}
							target={gumballTargetRef.current}
							onDragEnd={onGumballDragEnd}
							onDraggingChanged={setGumballDragActive}
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
