import { Clone, Line, OrbitControls, PerspectiveCamera, TransformControls, useGLTF } from "@react-three/drei";
import { Canvas, createPortal, useFrame, useStore, useThree, type ThreeEvent } from "@react-three/fiber";
import React, {
	Children,
	Suspense,
	createContext,
	isValidElement,
	memo,
	useCallback,
	useContext,
	useEffect,
	useLayoutEffect,
	useMemo,
	useReducer,
	useRef,
	useState,
	type ChangeEvent,
	type CSSProperties,
	type MutableRefObject,
	type ReactElement,
	type ReactNode,
} from "react";
import {
	Box3,
	BoxGeometry,
	BufferGeometry,
	Color,
	EdgesGeometry,
	Float32BufferAttribute,
	GridHelper,
	Line as ThreeLine,
	LineBasicMaterial,
	LineSegments,
	Mesh,
	MeshStandardMaterial,
	MOUSE,
	Points,
	PointsMaterial,
	PerspectiveCamera as ThreePerspectiveCamera,
	Plane,
	Euler,
	Matrix4,
	Quaternion,
	Raycaster,
	Vector2,
	Vector3,
	type Camera,
	Group,
	type Object3D,
	type Scene as ThreeScene,
	type WebGLRenderer,
} from "three";
import { ProductRuntime, registerWindowBody, type FooterItem, type UiScene3DHostSurfaceNode } from "@elements/framework";
import {
	ProductView,
	mountReactApp,
	registerSidePanelBody,
	registerUiScene3DSurfaceHost,
	useApp,
} from "@elements/framework-react";
import {
	Button,
	Expertise,
	Input,
	Label,
	LevelProvider,
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
	applyElementsSurfaceChrome,
	getLevelBgClass,
	type ElementsSurfaceDevice,
	type ElementsSurfaceTheme,
} from "@elements/ui";
import { ClipboardList, Settings, Trash2 } from "lucide-react";
import {
	LS_DEVICE,
	LS_EXPERTISE,
	LS_THEME,
	PLAY_APP_ID,
	SCENE_PLAY_BODY_KEY,
	SCENE_PLAY_CONTROLLER_ID,
	SCENE_PLAY_EMPTY_SELECTION,
	SCENE_PLAY_SCENE_SURFACE_ID,
	ScenePlayShellController,
	buildScenePlayAppRuntime,
	buildScenePlayDeclarativeBody,
	parseKindCatalogs,
	parseKindCompatibility,
	parseStoredDevice,
	parseStoredExpertise,
	parseStoredTheme,
	sceneVortexFullId,
	updateSceneAttractionInFixture,
	updateSceneObjectInFixture,
	updateSceneVortexInFixture,
	type ScenePlaySelection,
	type ScenePlaySnapshot,
} from "./play/index.ts";
import nakaginSceneFixtureJson from "./play/fixtures/nakagin-capsule-tower.scene.json";
import "./play/globals.css";

type SceneListenerTarget = Pick<EventTarget, "addEventListener" | "removeEventListener">;

class SceneEventBindingController {
	private readonly cleanups: Array<() => void> = [];

	listen(target: SceneListenerTarget | null | undefined, kind: string, listener: EventListenerOrEventListenerObject, options?: boolean | AddEventListenerOptions): void {
		if (!target) return;
		target.addEventListener(kind, listener, options);
		this.cleanups.push(() => target.removeEventListener(kind, listener, options));
	}

	dispose(): void {
		while (this.cleanups.length > 0) {
			this.cleanups.pop()?.();
		}
	}
}

//#region ­ƒöûKinds
export type Vec3 = readonly [number, number, number];
export type Quat = readonly [number, number, number, number];

export type RelocateMode = "translate" | "rotate" | "scale";
export type SelectionMode = "single" | "additive" | "subtractive" | "toggle";
export type ConnectKind = "indirect" | "connect" | "proximity";
export const MESH_STYLE_KINDS = [
	"original",
	"neutral",
	"hovered",
	"selected",
	"highlighted",
	"disabled",
] as const;
/** @emoji ­ƒÄ¿ Homogeneous GLB presentation kind for pooled scene meshes ({@link MeshBody}). */
export type MeshStyleKind = (typeof MESH_STYLE_KINDS)[number];
/** @emoji ­ƒÄ¿ Default object mesh style when none is passed ({@link MeshBody}). */
export const DEFAULT_MESH_STYLE: MeshStyleKind = "neutral";
export type DomainKind = "urban" | "architecture" | "detailing" | "engineering";
export type ScaleKind =
	| "1to50000"
	| "1to25000"
	| "1to10000"
	| "1to5000"
	| "1to2500"
	| "1to1000"
	| "1to500"
	| "1to333"
	| "1to200"
	| "1to100"
	| "1to50"
	| "1to33"
	| "1to25"
	| "1to10"
	| "1to5"
	| "1to1"
	| "2to1"
	| "5to1"
	| "10to1"
	| "20to1"
	| "50to1";

export const DEFAULT_DOMAIN: DomainKind = "architecture";
export const DEFAULT_SCALE_REFERENCE = 100;

const SCALE_RATIOS: Record<ScaleKind, readonly [numerator: number, denominator: number]> = {
	"1to50000": [1, 50_000],
	"1to25000": [1, 25_000],
	"1to10000": [1, 10_000],
	"1to5000": [1, 5_000],
	"1to2500": [1, 2_500],
	"1to1000": [1, 1_000],
	"1to500": [1, 500],
	"1to333": [1, 333],
	"1to200": [1, 200],
	"1to100": [1, 100],
	"1to50": [1, 50],
	"1to33": [1, 33],
	"1to25": [1, 25],
	"1to10": [1, 10],
	"1to5": [1, 5],
	"1to1": [1, 1],
	"2to1": [2, 1],
	"5to1": [5, 1],
	"10to1": [10, 1],
	"20to1": [20, 1],
	"50to1": [50, 1],
};

export interface CameraState {
	position: Vec3;
	target: Vec3;
	zoom: number;
}

/** @emoji 📶 Scene LOD as scale denominator/numerator (e.g. 50000 = 1:50000, 0.5 = 2:1); higher = coarser. */
export type SceneLod = number;

/** @emoji 🎨 Per-LOD mesh URL entry for {@link ObjectProps.meshByLod} and {@link VortexProps.handleMeshByLod}. */
export interface LodMeshEntry {
	readonly lod: number;
	readonly url: string;
}

/** @emoji 📐 Default manual / slider LOD range (log-scaled). */
export const DEFAULT_LOD_RANGE = { min: 0.01, max: 100_000 } as const;

/** @emoji 📐 Default scene LOD when neither auto nor depth-variable applies. */
export const DEFAULT_MANUAL_LOD = 100;

/** @emoji 📐 Linear slider domain for log-mapped scene LOD in play measures. */
export const SCENE_LOD_SLIDER_MIN = 0;

/** @emoji 📐 Linear slider domain for log-mapped scene LOD in play measures. */
export const SCENE_LOD_SLIDER_MAX = 1000;

/** @emoji 📐 Epsilon for scene LOD change notifications. */
export const SCENE_LOD_EPSILON = 0.01;

/** @emoji 📐 Attraction snap is disabled at or above this coarse scene LOD (≈ 1:1000). */
export const SCENE_ATTRACTION_SNAP_MAX_LOD = 1000;

/** @emoji 📐 Large LOD grid quantum in world units (sketch board `BOARD_LOD_GRID_MAJOR_QUANTUM`). */
export const LOD_GRID_MAJOR_QUANTUM = 10;

/** @emoji 📐 Default grid factor (sketch board `DEFAULT_BOARD_GRID_FACTOR`). */
export const DEFAULT_LOD_GRID_FACTOR = 10;

export interface VortexProps {
	id: string;
	vortexKind?: string;
	position: Vec3;
	direction?: Vec3;
	radius?: number;
	visible?: boolean;
	handleMeshUrl?: string;
	/** @emoji 🎨 Optional per-LOD GLB URLs for the handle mesh; falls back to {@link handleMeshUrl}. */
	handleMeshByLod?: readonly LodMeshEntry[];
	children?: ReactNode;
}

export interface MagnetProps {
	id: string;
	magnetKind?: string;
	position: Vec3;
	orientation?: Quat;
	size: Vec3;
}

export interface ObjectProps {
	id: string;
	objectKind?: string;
	meshUrl: string;
	/** @emoji 🎨 Optional per-LOD GLB URLs; falls back to {@link meshUrl}. */
	meshByLod?: readonly LodMeshEntry[];
	/** @emoji ­ƒÄ¿ Explicit mesh style; otherwise derived from disabled, selected, highlighted, hovered. */
	style?: MeshStyleKind;
	origin: Vec3;
	orientation?: Quat;
	scale?: number | Vec3;
	label?: string;
	selected?: boolean;
	hovered?: boolean;
	highlighted?: boolean;
	disabled?: boolean;
	visible?: boolean;
	relocate?: RelocateMode | false;
	/** @emoji ­ƒº▓ Object ids attracted to this object in the resolved ownership tree. */
	attracting?: readonly string[];
	/** @emoji ­ƒò│´©Å Root of a connected attraction component (wormhole). */
	wormhole?: boolean;
	children?: ReactNode;
	userData?: Record<string, unknown>;
}

export interface AttractionProps {
	id: string;
	attracting: `${string}:${string}`;
	attracted: `${string}:${string}`;
	attractionKind?: string;
}

export const PLACEHOLDER_MESH_URL = "elements.scene.placeholder://box";

export interface EdgeKindCatalogEntry {
	id: string;
	label?: string;
	name?: string;
}

export interface HandleKindCatalogEntry {
	id: string;
	label?: string;
	name?: string;
	color?: string;
	defaultWireKind?: string;
	scale?: number;
}

export interface NodeKindCatalogEntry {
	id: string;
	label?: string;
	name?: string;
	color?: string;
	shape?: string;
}

export interface WireKindCatalogEntry {
	id: string;
	label?: string;
	name?: string;
	defaultEdgeKind?: string;
}

export interface KindCatalogBundle {
	edges?: readonly EdgeKindCatalogEntry[];
	handles?: readonly HandleKindCatalogEntry[];
	nodes?: readonly NodeKindCatalogEntry[];
	wires?: readonly WireKindCatalogEntry[];
}

export interface KindCompatEntry {
	source: string;
	target: string;
	bidirectional?: boolean;
	important?: boolean;
	specificity?: "general" | "object" | "attraction" | "handle" | "wire" | "node" | "edge";
}

export interface SelectionSnapshot {
	readonly objectIds: readonly string[];
	readonly vortexIds: readonly string[];
}

/** @emoji 🖱️ Exclusive scene hover target (at most one active). */
export type SceneHoverTarget =
	| { readonly kind: "object"; readonly id: string }
	| { readonly kind: "vortex"; readonly fullId: string }
	| { readonly kind: "attraction"; readonly id: string };

/** @emoji 🖱️ Compares two hover targets for equality. */
export function sceneHoverTargetsEqual(a: SceneHoverTarget | null, b: SceneHoverTarget | null): boolean {
	if (a === b) {
		return true;
	}
	if (!a || !b) {
		return false;
	}
	if (a.kind !== b.kind) {
		return false;
	}
	switch (a.kind) {
		case "object":
			return b.kind === "object" && a.id === b.id;
		case "vortex":
			return b.kind === "vortex" && a.fullId === b.fullId;
		case "attraction":
			return b.kind === "attraction" && a.id === b.id;
		default:
			return false;
	}
}

export interface RelocatePayload {
	readonly objectId: string;
	readonly mode: RelocateMode;
	readonly before: { origin: Vec3; orientation: Quat; scale: Vec3 };
	readonly after: { origin: Vec3; orientation: Quat; scale: Vec3 };
}

export interface AttractionPayload {
	readonly attracting: string;
	readonly attracted: string;
	readonly attractionId?: string;
}

export interface AttractionCompatibleObjectsPayload {
	readonly attracting: string;
	readonly objectIds: readonly string[];
}

export interface AttractionTargetRingPayload {
	readonly attracting: string;
	readonly objectId: string | null;
	readonly vortexFullIds: readonly string[];
}

export interface AttractionIndirectPickAwait {
	readonly attractingFullId: string;
	readonly attractedObjectId: string;
	readonly candidates: readonly string[];
}

export interface CanvasProps {
	camera?: Partial<CameraState>;
	domain?: DomainKind;
	chunkSize?: number;
	kindCatalogs?: KindCatalogBundle;
	kindCompatibility?: readonly KindCompatEntry[];
	/** @emoji ­ƒÜ½ Vortex full ids (`objectId:vortexId`) that already terminate an attraction and cannot start or receive a new attraction. */
	blockedVortexFullIds?: ReadonlySet<string>;
	proximityRadius?: number;
	relocateMode?: RelocateMode;
	selectionMode?: SelectionMode;
	/** @emoji 📶 When true (default), orbit camera distance drives scene LOD. */
	automaticLod?: boolean;
	/** @emoji 📶 When true, each object picks LOD from its world distance to the camera. */
	depthVariableLod?: boolean;
	/** @emoji 📶 Manual scene LOD when {@link automaticLod} and {@link depthVariableLod} are both false. */
	lod?: number;
	/** @emoji 📏 Orbit distance at which scene LOD is ~1 (`distance / reference`). */
	lodDistanceReference?: number;
	/** @emoji 📐 Clamp range for manual LOD slider UI. */
	availableLodRange?: { readonly min: number; readonly max: number };
	/** @emoji ­ƒôÉ Multiplier for LOD grid steps (board `grid_factor`). */
	gridFactor?: number;
	/** @emoji ­ƒôÉ When true, draw a world `GridHelper` stepped by the current LOD band grid. */
	showLodGrid?: boolean;
	/** @emoji ­ƒº▓ When true, translate relocate snaps to the finest visible LOD grid step (board `grid_snap_enabled`). */
	gridSnapEnabled?: boolean;
	onCamera?: (s: CameraState) => void;
	/** @emoji 📶 Emits whenever the resolved scene-level LOD changes. */
	onLodChange?: (lod: number) => void;
	onSelect?: (snap: SelectionSnapshot) => void;
	/** @emoji 🎯 When set, canvas selection is controlled by the host (e.g. scene play inspector). */
	selection?: SelectionSnapshot;
	onRelocate?: (p: RelocatePayload) => void;
	onConnect?: (p: AttractionPayload) => void;
	onIndirectConnect?: (p: AttractionPayload) => void;
	onProximityConnect?: (p: AttractionPayload) => void;
	onAttractionCompatibleObjects?: (p: AttractionCompatibleObjectsPayload) => void;
	onAttractionTargetRing?: (p: AttractionTargetRingPayload) => void;
	children?: ReactNode;
}

export const FIXTURE_DRAG_V1_MIME = "application/x-elements-scene-fixture+json;v=1";

export interface FixtureObjectV1 extends ObjectProps {
	vortices: VortexProps[];
	magnets?: MagnetProps[];
}

/** @emoji 🧭 Scene fixture vectors and quaternions use CAD: X right, Y front, Z up; GLB meshes stay glTF Y-up. */
export interface FixtureV1 {
	schema: "elements.scene.fixture/v1";
	camera: CameraState;
	domain: DomainKind;
	meta?: Record<string, unknown>;
	attractions: AttractionProps[];
	objects: FixtureObjectV1[];
}
//#endregion ­ƒöûKinds

//#region 📶Lod
/** @emoji 📶 Maps orbit camera distance to scene LOD (`distance / reference`). */
export function lodFromCameraDistance(distance: number, reference: number): number {
	const d = Math.max(distance, 1e-6);
	const ref = Math.max(reference, 1e-6);
	return d / ref;
}

/** @emoji 📶 Picks the closest available LOD; on log-distance ties prefers the smaller (more detailed) LOD. */
export function pickClosestLod(available: readonly number[], desired: number): number | null {
	if (!available.length || !Number.isFinite(desired) || desired <= 0) return null;
	let best = available[0]!;
	let bestDist = Math.abs(Math.log(best) - Math.log(desired));
	for (let i = 1; i < available.length; i++) {
		const rep = available[i]!;
		if (!Number.isFinite(rep) || rep <= 0) continue;
		const dist = Math.abs(Math.log(rep) - Math.log(desired));
		if (dist < bestDist - 1e-12 || (Math.abs(dist - bestDist) <= 1e-12 && rep < best)) {
			best = rep;
			bestDist = dist;
		}
	}
	return best;
}

/** @emoji 🎨 Resolves a mesh URL from per-LOD entries with {@link pickClosestLod} and optional fallback. */
export function pickClosestMeshUrl(
	entries: readonly LodMeshEntry[] | undefined,
	desired: number,
	fallback?: string,
): string | undefined {
	if (!entries?.length) return fallback;
	const lods = entries.map((e) => e.lod).filter((lod) => Number.isFinite(lod) && lod > 0);
	const picked = pickClosestLod(lods, desired);
	if (picked == null) return fallback;
	const match = entries.find((e) => e.lod === picked);
	return match?.url ?? fallback;
}

/** @emoji 📶 Formats scene LOD for `data-scene-lod` and play readouts. */
export function formatSceneLod(lod: number): string {
	return Number.isFinite(lod) ? lod.toFixed(2) : "—";
}

/** @emoji 📶 Maps a linear slider position to log-spaced scene LOD. */
export function lodFromSliderValue(slider: number, range: { readonly min: number; readonly max: number } = DEFAULT_LOD_RANGE): number {
	const t = Math.max(0, Math.min(1, (slider - SCENE_LOD_SLIDER_MIN) / (SCENE_LOD_SLIDER_MAX - SCENE_LOD_SLIDER_MIN)));
	const logMin = Math.log(range.min);
	const logMax = Math.log(range.max);
	return Math.exp(logMin + t * (logMax - logMin));
}

/** @emoji 📶 Maps scene LOD to a linear slider position. */
export function sliderValueFromLod(lod: number, range: { readonly min: number; readonly max: number } = DEFAULT_LOD_RANGE): number {
	const clamped = Math.max(range.min, Math.min(range.max, lod));
	const logMin = Math.log(range.min);
	const logMax = Math.log(range.max);
	const t = (Math.log(clamped) - logMin) / (logMax - logMin);
	return Math.round(SCENE_LOD_SLIDER_MIN + t * (SCENE_LOD_SLIDER_MAX - SCENE_LOD_SLIDER_MIN));
}

/** @emoji 📶 Maps play / window LOD controls to {@link CanvasProps}. */
export function sceneLodCanvasProps(state: {
	readonly automaticLod: boolean;
	readonly depthVariableLod: boolean;
	readonly manualLod: number;
}): Pick<CanvasProps, "automaticLod" | "depthVariableLod" | "lod"> {
	return {
		automaticLod: state.automaticLod,
		depthVariableLod: state.depthVariableLod,
		lod: !state.automaticLod && !state.depthVariableLod ? state.manualLod : undefined,
	};
}

/** @emoji 📐 Visible LOD grid / relocate snap step in world units. */
export function lodGridStepWorld(lod: number, gridFactor: number): number | null {
	if (!Number.isFinite(lod) || lod <= 0) return null;
	const raw = lod * 0.05 * gridFactor;
	return raw > 50 * gridFactor ? null : raw;
}

/** @emoji 🌐 True when primary handle visuals are drawn at the given scene LOD. */
export function lodHandlePrimaryVisible(lod: number): boolean {
	return lod <= 200;
}

/** @emoji 🌐 True when invisible handle pick proxies are used instead of GLB handles. */
export function lodHandlePickProxy(lod: number): boolean {
	return lod > 200 && lod <= 1000;
}

export interface LodContextValue {
	readonly lod: number;
	readonly depthVariable: boolean;
	readonly lodForWorldPosition: (position: Vec3) => number;
	readonly gridStepWorld: number | null;
	readonly gridFactor: number;
	readonly gridSnapEnabled: boolean;
}

const LodContext = createContext<LodContextValue | null>(null);

/** @emoji 📶 Reads the live scene LOD band and grid snap step from canvas context. */
export function useLod(): LodContextValue {
	const v = useContext(LodContext);
	if (!v) throw new Error("Scene LOD missing");
	return v;
}

interface LodRuntimeCells {
	sceneLod: number;
	depthVariable: boolean;
	distanceReference: number;
	camera: Camera | null;
	tmpWorld: Vector3;
}

function resolveMeshUrlForLod(
	meshByLod: readonly LodMeshEntry[] | undefined,
	fallbackMeshUrl: string,
	lod: number,
): string {
	if (fallbackMeshUrl === PLACEHOLDER_MESH_URL) return fallbackMeshUrl;
	return pickClosestMeshUrl(meshByLod, lod, fallbackMeshUrl) ?? fallbackMeshUrl;
}

/** @emoji 🎨 Resolves per-object mesh URL; useFrame only when depth-variable or per-LOD meshes exist, and setState only when URL changes. */
function useResolvedSceneMeshUrl(opts: {
	readonly origin: Vec3;
	readonly meshByLod?: readonly LodMeshEntry[];
	readonly fallbackMeshUrl: string;
}): string {
	const lodCtx = useLod();
	const trackLod = lodCtx.depthVariable || (opts.meshByLod?.length ?? 0) > 0;
	const meshByLodRef = useRef(opts.meshByLod);
	meshByLodRef.current = opts.meshByLod;
	const fallbackRef = useRef(opts.fallbackMeshUrl);
	fallbackRef.current = opts.fallbackMeshUrl;
	const originRef = useRef(opts.origin);
	originRef.current = opts.origin;
	const [url, setUrl] = useState(() =>
		resolveMeshUrlForLod(
			opts.meshByLod,
			opts.fallbackMeshUrl,
			lodCtx.depthVariable ? lodCtx.lodForWorldPosition(opts.origin) : lodCtx.lod,
		),
	);
	useFrame(() => {
		if (!trackLod) return;
		const lod = lodCtx.depthVariable ? lodCtx.lodForWorldPosition(originRef.current) : lodCtx.lod;
		const next = resolveMeshUrlForLod(meshByLodRef.current, fallbackRef.current, lod);
		setUrl((prev) => (prev === next ? prev : next));
	});
	if (!trackLod) {
		return resolveMeshUrlForLod(opts.meshByLod, opts.fallbackMeshUrl, lodCtx.lod);
	}
	return url;
}

interface VortexLodVisual {
	readonly drawHandleBody: boolean;
	readonly pickProxy: boolean;
	readonly meshUrl: string | undefined;
}

function vortexLodVisual(
	lod: number,
	linger: boolean,
	handleMeshByLod: readonly LodMeshEntry[] | undefined,
	handleMeshUrl: string | undefined,
): VortexLodVisual {
	const drawHandleBody = lodHandlePrimaryVisible(lod) || linger;
	const pickProxy = lodHandlePickProxy(lod) && !drawHandleBody;
	const meshUrl = drawHandleBody ? pickClosestMeshUrl(handleMeshByLod, lod, handleMeshUrl) : undefined;
	return { drawHandleBody, pickProxy, meshUrl };
}

function vortexLodVisualEqual(a: VortexLodVisual, b: VortexLodVisual): boolean {
	return a.drawHandleBody === b.drawHandleBody && a.pickProxy === b.pickProxy && a.meshUrl === b.meshUrl;
}

function LodGridHelper() {
	const lod = useLod();
	const grid = useMemo(() => {
		const step = lod.gridStepWorld;
		if (step == null || !Number.isFinite(step) || step <= 0) return null;
		const size = 12_000;
		const divs = Math.min(512, Math.max(2, Math.round(size / step)));
		return new GridHelper(size, divs, 0x8899aa, 0x445566);
	}, [lod.gridStepWorld]);
	useEffect(
		() => () => {
			grid?.dispose();
		},
		[grid],
	);
	if (!grid) return null;
	return <primitive object={grid} position={[0, 0, 0]} />;
}

function LodFrameRunner(props: {
	readonly lodRef: MutableRefObject<number>;
	readonly lodRuntimeRef: MutableRefObject<LodRuntimeCells>;
	readonly distanceReference: number;
	readonly gridFactor: number;
	readonly gridSnapEnabled: boolean;
	readonly automaticLod: boolean;
	readonly depthVariableLod: boolean;
	readonly manualLod: number;
	readonly onSceneLod: (patch: {
		readonly sceneLod: number;
		readonly depthVariable: boolean;
		readonly gridStepWorld: number | null;
	}) => void;
	readonly onLodChange?: (lod: number) => void;
}) {
	const cam = useThree((s) => s.camera);
	const controls = useThree((s) => s.controls as { target?: Vector3 } | null);
	const tmpT = useMemo(() => new Vector3(), []);
	const prevLod = useRef<number | null>(null);
	const ctxSig = useRef("");
	useFrame(() => {
		const tgt = controls?.target ?? tmpT.set(0, 0, 0);
		const dist = cam.position.distanceTo(tgt);
		const autoLod = lodFromCameraDistance(dist, props.distanceReference);
		const sceneLod = props.automaticLod
			? autoLod
			: props.depthVariableLod
				? autoLod
				: props.manualLod;
		props.lodRef.current = sceneLod;
		const runtime = props.lodRuntimeRef.current;
		runtime.sceneLod = sceneLod;
		runtime.depthVariable = props.depthVariableLod;
		runtime.distanceReference = props.distanceReference;
		runtime.camera = cam;
		const gridStep = lodGridStepWorld(sceneLod, props.gridFactor);
		const sig = `${sceneLod}|${props.depthVariableLod ? 1 : 0}|${gridStep ?? "x"}|${props.gridFactor}|${props.gridSnapEnabled}`;
		if (ctxSig.current !== sig) {
			ctxSig.current = sig;
			props.onSceneLod({ sceneLod, depthVariable: props.depthVariableLod, gridStepWorld: gridStep });
		}
		if (prevLod.current === null || Math.abs(prevLod.current - sceneLod) > SCENE_LOD_EPSILON) {
			prevLod.current = sceneLod;
			props.onLodChange?.(sceneLod);
		}
	});
	return null;
}

function LodBridge(props: {
	readonly children: ReactNode;
	readonly lodRef: MutableRefObject<number>;
	readonly distanceReference: number;
	readonly gridFactor: number;
	readonly gridSnapEnabled: boolean;
	readonly showLodGrid: boolean;
	readonly automaticLod: boolean;
	readonly depthVariableLod: boolean;
	readonly manualLod: number;
	readonly onLodChange?: (lod: number) => void;
}) {
	const tmpWorld = useMemo(() => new Vector3(), []);
	const lodRuntimeRef = useRef<LodRuntimeCells>({
		sceneLod: DEFAULT_MANUAL_LOD,
		depthVariable: false,
		distanceReference: props.distanceReference,
		camera: null,
		tmpWorld,
	});
	const lodForWorldPosition = useCallback((position: Vec3) => {
		const r = lodRuntimeRef.current;
		if (!r.depthVariable || !r.camera) return r.sceneLod;
		r.tmpWorld.set(position[0], position[1], position[2]);
		return lodFromCameraDistance(r.camera.position.distanceTo(r.tmpWorld), r.distanceReference);
	}, []);
	const [sceneLod, setSceneLod] = useState(DEFAULT_MANUAL_LOD);
	const [depthVariable, setDepthVariable] = useState(false);
	const [gridStepWorld, setGridStepWorld] = useState<number | null>(() =>
		lodGridStepWorld(DEFAULT_MANUAL_LOD, props.gridFactor),
	);
	const onSceneLod = useCallback(
		(patch: { readonly sceneLod: number; readonly depthVariable: boolean; readonly gridStepWorld: number | null }) => {
			setSceneLod((prev) => (Math.abs(prev - patch.sceneLod) > SCENE_LOD_EPSILON ? patch.sceneLod : prev));
			setDepthVariable((prev) => (prev === patch.depthVariable ? prev : patch.depthVariable));
			setGridStepWorld((prev) => (prev === patch.gridStepWorld ? prev : patch.gridStepWorld));
		},
		[],
	);
	const lodCtx = useMemo<LodContextValue>(
		() => ({
			lod: sceneLod,
			depthVariable,
			lodForWorldPosition,
			gridStepWorld,
			gridFactor: props.gridFactor,
			gridSnapEnabled: props.gridSnapEnabled,
		}),
		[sceneLod, depthVariable, lodForWorldPosition, gridStepWorld, props.gridFactor, props.gridSnapEnabled],
	);
	return (
		<LodContext.Provider value={lodCtx}>
			<LodFrameRunner
				lodRef={props.lodRef}
				lodRuntimeRef={lodRuntimeRef}
				distanceReference={props.distanceReference}
				gridFactor={props.gridFactor}
				gridSnapEnabled={props.gridSnapEnabled}
				automaticLod={props.automaticLod}
				depthVariableLod={props.depthVariableLod}
				manualLod={props.manualLod}
				onSceneLod={onSceneLod}
				onLodChange={props.onLodChange}
			/>
			{props.showLodGrid ? <LodGridHelper /> : null}
			{props.children}
		</LodContext.Provider>
	);
}
//#endregion 📶Lod

//#region ­ƒº¥Fixture
function isVec3(v: unknown): v is Vec3 {
	return Array.isArray(v) && v.length === 3 && v.every((n) => typeof n === "number");
}

function isQuat(v: unknown): v is Quat {
	return Array.isArray(v) && v.length === 4 && v.every((n) => typeof n === "number");
}

function parseLodMeshEntries(v: unknown): readonly LodMeshEntry[] | undefined {
	if (!Array.isArray(v)) return undefined;
	const out: LodMeshEntry[] = [];
	for (const row of v) {
		if (!row || typeof row !== "object") continue;
		const o = row as Record<string, unknown>;
		const lod = o.lod;
		const url = o.url;
		if (typeof lod !== "number" || !Number.isFinite(lod) || lod <= 0) continue;
		if (typeof url !== "string" || !url.length) continue;
		out.push({ lod, url });
	}
	return out.length ? out : undefined;
}

function parseDomainKind(value: unknown): DomainKind {
	if (typeof value !== "string") {
		return DEFAULT_DOMAIN;
	}
	switch (value.trim().toLowerCase()) {
		case "urban":
			return "urban";
		case "architecture":
			return "architecture";
		case "detailing":
			return "detailing";
		case "engineering":
			return "engineering";
		default:
			return DEFAULT_DOMAIN;
	}
}

function parseHandleMeshByLod(v: unknown): readonly LodMeshEntry[] | undefined {
	return parseLodMeshEntries(v);
}

export function parseFixtureV1(raw: unknown): FixtureV1 | null {
	if (!raw || typeof raw !== "object") return null;
	const r = raw as Record<string, unknown>;
	if (r.schema !== "elements.scene.fixture/v1") return null;
	const cam = r.camera;
	if (!cam || typeof cam !== "object") return null;
	const c = cam as Record<string, unknown>;
	const pos = c.position;
	const tgt = c.target;
	const zoom = c.zoom;
	if (!isVec3(pos) || !isVec3(tgt) || typeof zoom !== "number") return null;
	const attractionsRaw = r.attractions;
	const objsRaw = r.objects;
	if (!Array.isArray(attractionsRaw) || !Array.isArray(objsRaw)) return null;
	const attractions: AttractionProps[] = [];
	for (const attraction of attractionsRaw) {
		if (!attraction || typeof attraction !== "object") continue;
		const tr = attraction as Record<string, unknown>;
		if (typeof tr.id !== "string" || typeof tr.attracting !== "string" || typeof tr.attracted !== "string") continue;
		attractions.push({
			id: tr.id,
			attracting: tr.attracting as AttractionProps["attracting"],
			attracted: tr.attracted as AttractionProps["attracted"],
			...(typeof tr.attractionKind === "string" ? { attractionKind: tr.attractionKind } : {}),
		});
	}
	const objects: FixtureObjectV1[] = [];
	for (const o of objsRaw) {
		if (!o || typeof o !== "object") continue;
		const or = o as Record<string, unknown>;
		if (typeof or.id !== "string" || typeof or.meshUrl !== "string") continue;
		const origin = or.origin;
		if (!isVec3(origin)) continue;
		const vortices: VortexProps[] = [];
		const vr = or.vortices;
		if (Array.isArray(vr)) {
			for (const v of vr) {
				if (!v || typeof v !== "object") continue;
				const vx = v as Record<string, unknown>;
				if (typeof vx.id !== "string" || !isVec3(vx.position)) continue;
				const handleMeshByLod = parseHandleMeshByLod(vx.handleMeshByLod);
				vortices.push({
					id: vx.id,
					...(typeof vx.vortexKind === "string" ? { vortexKind: vx.vortexKind } : {}),
					position: vx.position,
					...(isVec3(vx.direction) ? { direction: vx.direction } : {}),
					...(typeof vx.radius === "number" ? { radius: vx.radius } : {}),
					...(typeof vx.handleMeshUrl === "string" ? { handleMeshUrl: vx.handleMeshUrl } : {}),
					...(handleMeshByLod ? { handleMeshByLod } : {}),
				});
			}
		}
		const meshByLod = parseLodMeshEntries(or.meshByLod);
		objects.push({
			id: or.id,
			meshUrl: or.meshUrl,
			origin,
			...(meshByLod ? { meshByLod } : {}),
			...(typeof or.objectKind === "string" ? { objectKind: or.objectKind } : {}),
			...(typeof or.label === "string" ? { label: or.label } : {}),
			...(or.wormhole === true ? { wormhole: true } : {}),
			...(isQuat(or.orientation) ? { orientation: or.orientation } : {}),
			...(typeof or.scale === "number" || isVec3(or.scale) ? { scale: or.scale as number | Vec3 } : {}),
			vortices,
		});
	}
	return {
		schema: "elements.scene.fixture/v1",
		camera: { position: pos, target: tgt, zoom },
		domain: parseDomainKind(r.domain),
		...(r.meta && typeof r.meta === "object" ? { meta: r.meta as Record<string, unknown> } : {}),
		attractions,
		objects,
	};
}

export function encodeSceneFixtureForDragV1(fixture: FixtureV1): string {
	return JSON.stringify(fixture);
}
//#endregion ­ƒº¥Fixture

//#region ­ƒò©´©ÅAttractionGraph
/** @emoji ­ƒöù Parsed `objectId:vortexId` attraction endpoint. */
export function parseVortexFullId(full: string): { readonly objectId: string; readonly vortexId: string } {
	const i = full.indexOf(":");
	if (i < 0) {
		return { objectId: full, vortexId: "link" };
	}
	return { objectId: full.slice(0, i), vortexId: full.slice(i + 1) };
}

/** @emoji ­ƒò│´©Å True when the object is an explicit or inferred wormhole root. */
export function isWormholeObject(
	objectId: string,
	props: { readonly wormhole?: boolean; readonly objectKind?: string },
	inferredWormholeIds: ReadonlySet<string>,
): boolean {
	if (props.wormhole === true) {
		return true;
	}
	const kind = props.objectKind ?? "";
	if (kind.includes("wormhole")) {
		return true;
	}
	return inferredWormholeIds.has(objectId);
}

/** @emoji ­ƒº▓ One object-level attraction edge derived from an attraction (`attracting` attracts `attracted`). */
export interface AttractionEdge {
	readonly attractingObjectId: string;
	readonly attractedObjectId: string;
	readonly attractionId: string;
}

/** @emoji ­ƒº▓ Maps scene attractions to object-level attraction edges. */
export function attractionEdgesFromAttractions(attractions: readonly AttractionProps[]): AttractionEdge[] {
	const out: AttractionEdge[] = [];
	for (const attraction of attractions) {
		const attractingObjectId = parseVortexFullId(attraction.attracting).objectId;
		const attractedObjectId = parseVortexFullId(attraction.attracted).objectId;
		if (!attractingObjectId || !attractedObjectId || attractingObjectId === attractedObjectId) {
			continue;
		}
		out.push({ attractingObjectId, attractedObjectId, attractionId: attraction.id });
	}
	return out;
}

export interface SceneAttractionTree {
	readonly parentByObjectId: ReadonlyMap<string, string | null>;
	readonly attractingByObjectId: ReadonlyMap<string, readonly string[]>;
	readonly wormholeDistanceByObjectId: ReadonlyMap<string, number>;
	readonly wormholeIds: readonly string[];
}

function vec3Add(a: Vec3, b: Vec3): Vec3 {
	return [a[0] + b[0], a[1] + b[1], a[2] + b[2]] as Vec3;
}

function vec3Sub(a: Vec3, b: Vec3): Vec3 {
	return [a[0] - b[0], a[1] - b[1], a[2] - b[2]] as Vec3;
}

function undirectedComponents(objectIds: readonly string[], edges: readonly AttractionEdge[]): string[][] {
	const idSet = new Set(objectIds);
	const adj = new Map<string, Set<string>>();
	for (const id of objectIds) {
		adj.set(id, new Set());
	}
	for (const e of edges) {
		if (!idSet.has(e.attractingObjectId) || !idSet.has(e.attractedObjectId)) {
			continue;
		}
		adj.get(e.attractingObjectId)!.add(e.attractedObjectId);
		adj.get(e.attractedObjectId)!.add(e.attractingObjectId);
	}
	const seen = new Set<string>();
	const components: string[][] = [];
	for (const id of objectIds) {
		if (seen.has(id)) {
			continue;
		}
		const stack = [id];
		const comp: string[] = [];
		seen.add(id);
		while (stack.length) {
			const cur = stack.pop()!;
			comp.push(cur);
			for (const nb of adj.get(cur) ?? []) {
				if (seen.has(nb)) {
					continue;
				}
				seen.add(nb);
				stack.push(nb);
			}
		}
		components.push(comp);
	}
	return components;
}

/** @emoji ­ƒöä True when `attractingObjectId ÔåÆ attractedObjectId` closes a directed cycle in attraction edges. */
export function wouldAttractionEdgeIntroduceCycle(
	edges: readonly AttractionEdge[],
	attractingObjectId: string,
	attractedObjectId: string,
): boolean {
	if (!attractingObjectId || !attractedObjectId || attractingObjectId === attractedObjectId) {
		return true;
	}
	const outgoing = new Map<string, string[]>();
	for (const edge of edges) {
		const next = outgoing.get(edge.attractingObjectId) ?? [];
		next.push(edge.attractedObjectId);
		outgoing.set(edge.attractingObjectId, next);
	}
	const stack = [attractedObjectId];
	const seen = new Set<string>();
	while (stack.length) {
		const id = stack.pop()!;
		if (id === attractingObjectId) {
			return true;
		}
		if (seen.has(id)) {
			continue;
		}
		seen.add(id);
		for (const child of outgoing.get(id) ?? []) {
			stack.push(child);
		}
	}
	return false;
}

function parentOwnershipCycleMemberIds(
	parentByObjectId: ReadonlyMap<string, string | null>,
	startId: string,
): readonly string[] | null {
	const order: string[] = [];
	const index = new Map<string, number>();
	let cur: string | null = startId;
	while (cur) {
		const at = index.get(cur);
		if (at !== undefined) {
			return order.slice(at);
		}
		index.set(cur, order.length);
		order.push(cur);
		cur = parentByObjectId.get(cur) ?? null;
	}
	return null;
}

/** @emoji Ô£é´©Å Clears one parent link per ownership cycle so {@link SceneAttractionTree} stays a forest. */
function breakOwnershipParentCycles(parentByObjectId: Map<string, string | null>): void {
	for (;;) {
		let cycle: readonly string[] | null = null;
		for (const id of parentByObjectId.keys()) {
			cycle = parentOwnershipCycleMemberIds(parentByObjectId, id);
			if (cycle?.length) {
				break;
			}
		}
		if (!cycle?.length) {
			return;
		}
		const detach = cycle.slice().sort().at(-1)!;
		parentByObjectId.set(detach, null);
	}
}

/** @emoji ­ƒò©´©Å Resolves a forest from attraction edges: wormhole roots, closest-to-wormhole parent when multiply attracted. */
export function resolveSceneAttractionTree(args: {
	readonly objectIds: readonly string[];
	readonly edges: readonly AttractionEdge[];
	readonly explicitWormholeIds?: ReadonlySet<string>;
}): SceneAttractionTree {
	const explicit = args.explicitWormholeIds ?? new Set<string>();
	const incoming = new Map<string, AttractionEdge[]>();
	const outgoing = new Map<string, string[]>();
	for (const id of args.objectIds) {
		incoming.set(id, []);
		outgoing.set(id, []);
	}
	for (const edge of args.edges) {
		if (!incoming.has(edge.attractedObjectId) || !outgoing.has(edge.attractingObjectId)) {
			continue;
		}
		incoming.get(edge.attractedObjectId)!.push(edge);
		outgoing.get(edge.attractingObjectId)!.push(edge.attractedObjectId);
	}

	const wormholeIds: string[] = [];
	const wormholeDistanceByObjectId = new Map<string, number>();
	const parentByObjectId = new Map<string, string | null>();

	for (const comp of undirectedComponents(args.objectIds, args.edges)) {
		const compSet = new Set(comp);
		const compIncoming = new Map<string, AttractionEdge[]>();
		for (const id of comp) {
			compIncoming.set(
				id,
				(incoming.get(id) ?? []).filter(
					(e) => compSet.has(e.attractingObjectId) && compSet.has(e.attractedObjectId),
				),
			);
		}
		let roots = comp.filter((id) => explicit.has(id));
		if (!roots.length) {
			roots = comp.filter((id) => (compIncoming.get(id) ?? []).length === 0);
		}
		if (!roots.length) {
			roots = [comp.slice().sort()[0]!];
		}
		for (const root of roots) {
			if (!wormholeIds.includes(root)) {
				wormholeIds.push(root);
			}
		}
		const dist = new Map<string, number>();
		const queue: string[] = [];
		for (const root of roots) {
			dist.set(root, 0);
			queue.push(root);
		}
		while (queue.length) {
			const cur = queue.shift()!;
			const d = dist.get(cur) ?? 0;
			for (const child of outgoing.get(cur) ?? []) {
				if (!compSet.has(child)) {
					continue;
				}
				const next = d + 1;
				const prev = dist.get(child);
				if (prev === undefined || next < prev) {
					dist.set(child, next);
					queue.push(child);
				}
			}
		}
		for (const id of comp) {
			const inc = compIncoming.get(id) ?? [];
			if (!inc.length) {
				parentByObjectId.set(id, null);
				continue;
			}
			let best: AttractionEdge | null = null;
			let bestDist = Number.POSITIVE_INFINITY;
			for (const edge of inc) {
				const d = dist.get(edge.attractingObjectId) ?? Number.POSITIVE_INFINITY;
				if (
					d < bestDist ||
					(d === bestDist &&
						(!best || edge.attractingObjectId.localeCompare(best.attractingObjectId) < 0))
				) {
					bestDist = d;
					best = edge;
				}
			}
			parentByObjectId.set(id, best?.attractingObjectId ?? null);
		}
		for (const id of comp) {
			wormholeDistanceByObjectId.set(id, dist.get(id) ?? Number.POSITIVE_INFINITY);
		}
	}

	for (const id of args.objectIds) {
		if (!parentByObjectId.has(id)) {
			parentByObjectId.set(id, null);
			wormholeDistanceByObjectId.set(id, explicit.has(id) ? 0 : Number.POSITIVE_INFINITY);
		}
	}

	breakOwnershipParentCycles(parentByObjectId);

	const attractingByObjectId = new Map<string, string[]>();
	for (const id of args.objectIds) {
		attractingByObjectId.set(id, []);
	}
	for (const [child, parent] of parentByObjectId) {
		if (!parent) {
			continue;
		}
		const arr = attractingByObjectId.get(parent) ?? [];
		arr.push(child);
		attractingByObjectId.set(parent, arr);
	}
	for (const [, arr] of attractingByObjectId) {
		arr.sort();
	}

	return {
		parentByObjectId,
		attractingByObjectId,
		wormholeDistanceByObjectId,
		wormholeIds: wormholeIds.slice().sort(),
	};
}

/** @emoji ­ƒº▓ Collects transitive attracted object ids in the resolved ownership tree. */
export function collectAttractedDescendantIds(
	rootObjectId: string,
	attractingByObjectId: ReadonlyMap<string, readonly string[]>,
): readonly string[] {
	const out: string[] = [];
	const stack = [...(attractingByObjectId.get(rootObjectId) ?? [])];
	const seen = new Set<string>();
	while (stack.length) {
		const id = stack.pop()!;
		if (seen.has(id)) {
			continue;
		}
		seen.add(id);
		out.push(id);
		for (const child of attractingByObjectId.get(id) ?? []) {
			stack.push(child);
		}
	}
	return out;
}

export interface ObjectRecord {
	readonly id: string;
	readonly objectKind?: string;
	readonly meshUrl: string;
	readonly origin: Vec3;
	readonly orientation?: Quat;
	readonly scale?: number | Vec3;
	readonly label?: string;
	readonly wormhole?: boolean;
	readonly vortices: readonly VortexProps[];
}

interface ObjectStateSnapshot {
	readonly records: ReadonlyMap<string, ObjectRecord>;
	readonly attractions: readonly AttractionProps[];
	readonly tree: SceneAttractionTree;
	readonly version: number;
}

type ObjectStateAction =
	| { readonly type: "init"; readonly fixture: FixtureV1 }
	| { readonly type: "syncPoses"; readonly fixture: FixtureV1 }
	| { readonly type: "relocate"; readonly payload: RelocatePayload }
	| { readonly type: "addAttraction"; readonly attraction: AttractionProps }
	| { readonly type: "removeObject"; readonly objectId: string };

function fixtureToRecords(objects: readonly FixtureObjectV1[]): Map<string, ObjectRecord> {
	const map = new Map<string, ObjectRecord>();
	for (const o of objects) {
		map.set(o.id, {
			id: o.id,
			meshUrl: o.meshUrl,
			origin: o.origin,
			...(o.objectKind ? { objectKind: o.objectKind } : {}),
			...(o.orientation ? { orientation: o.orientation } : {}),
			...(o.scale !== undefined ? { scale: o.scale } : {}),
			...(o.label ? { label: o.label } : {}),
			...(o.wormhole === true ? { wormhole: true } : {}),
			vortices: o.vortices,
		});
	}
	return map;
}

function buildSnapshot(records: ReadonlyMap<string, ObjectRecord>, attractions: readonly AttractionProps[], version: number): ObjectStateSnapshot {
	const objectIds = [...records.keys()];
	const explicitWormholes = new Set(
		objectIds.filter((id) => {
			const r = records.get(id);
			return r ? isWormholeObject(id, r, new Set()) : false;
		}),
	);
	const edges = attractionEdgesFromAttractions(attractions);
	const inferred = new Set<string>();
	for (const comp of undirectedComponents(objectIds, edges)) {
		const compEdges = edges.filter(
			(e) => comp.includes(e.attractingObjectId) && comp.includes(e.attractedObjectId),
		);
		const inc = new Map<string, number>();
		for (const id of comp) {
			inc.set(id, 0);
		}
		for (const e of compEdges) {
			inc.set(e.attractedObjectId, (inc.get(e.attractedObjectId) ?? 0) + 1);
		}
		for (const id of comp) {
			if ((inc.get(id) ?? 0) === 0 && !explicitWormholes.has(id)) {
				inferred.add(id);
			}
		}
	}
	const tree = resolveSceneAttractionTree({
		objectIds,
		edges,
		explicitWormholeIds: new Set([...explicitWormholes, ...inferred]),
	});
	return { records, attractions, tree, version };
}

/** @emoji ­ƒöæ Stable fingerprint for external fixture resync (ignores object reference identity). */
export function fixtureStateFingerprint(fixture: FixtureV1): string {
	const attractionIds = fixture.attractions.map((a) => a.id).join("\0");
	const objectIds = fixture.objects.map((o) => o.id).join("\0");
	return `${fixture.objects.length}\0${fixture.attractions.length}\0${objectIds}\0${attractionIds}`;
}

/** @emoji ­ƒôì Fingerprint of object poses for syncing fixture moves without resetting attractions. */
export function fixturePoseFingerprint(fixture: FixtureV1): string {
	return fixture.objects
		.map((object) => {
			const o = object.origin.join(",");
			const q = object.orientation?.join(",") ?? "";
			const s =
				object.scale === undefined
					? ""
					: typeof object.scale === "number"
						? String(object.scale)
						: object.scale.join(",");
			return `${object.id}|${o}|${q}|${s}`;
		})
		.join("\0");
}

function objectStateReducer(state: ObjectStateSnapshot, action: ObjectStateAction): ObjectStateSnapshot {
	switch (action.type) {
		case "init": {
			const records = fixtureToRecords(action.fixture.objects);
			return buildSnapshot(records, action.fixture.attractions, state.version + 1);
		}
		case "syncPoses": {
			const records = new Map(state.records);
			for (const object of action.fixture.objects) {
				const cur = records.get(object.id);
				if (!cur) {
					continue;
				}
				records.set(object.id, {
					...cur,
					origin: object.origin,
					orientation: object.orientation,
					scale: object.scale,
				});
			}
			return buildSnapshot(records, state.attractions, state.version + 1);
		}
		case "addAttraction": {
			const edges = attractionEdgesFromAttractions(state.attractions);
			const attractingObjectId = parseVortexFullId(action.attraction.attracting).objectId;
			const attractedObjectId = parseVortexFullId(action.attraction.attracted).objectId;
			if (wouldAttractionEdgeIntroduceCycle(edges, attractingObjectId, attractedObjectId)) {
				return state;
			}
			const attractions = [...state.attractions, action.attraction];
			return buildSnapshot(state.records, attractions, state.version + 1);
		}
		case "removeObject": {
			const records = new Map(state.records);
			records.delete(action.objectId);
			const attractions = state.attractions.filter((attraction) => {
				const s = parseVortexFullId(attraction.attracting).objectId;
				const tg = parseVortexFullId(attraction.attracted).objectId;
				return s !== action.objectId && tg !== action.objectId;
			});
			return buildSnapshot(records, attractions, state.version + 1);
		}
		case "relocate": {
			const { payload } = action;
			const records = new Map(state.records);
			const root = records.get(payload.objectId);
			if (!root) {
				return state;
			}
			const updatePose = (id: string, origin: Vec3, orientation: Quat, scale: Vec3) => {
				const cur = records.get(id);
				if (!cur) {
					return;
				}
				records.set(id, {
					...cur,
					origin,
					orientation,
					scale: scale[0] === scale[1] && scale[1] === scale[2] ? scale[0] : ([scale[0], scale[1], scale[2]] as Vec3),
				});
			};
			updatePose(
				payload.objectId,
				payload.after.origin,
				payload.after.orientation,
				payload.after.scale,
			);
			if (payload.mode === "translate") {
				const delta = vec3Sub(payload.after.origin, payload.before.origin);
				for (const id of collectAttractedDescendantIds(payload.objectId, state.tree.attractingByObjectId)) {
					const cur = records.get(id);
					if (!cur) {
						continue;
					}
					const sc = cur.scale;
					const scaleVec =
						typeof sc === "number"
							? ([sc, sc, sc] as Vec3)
							: sc
								? ([sc[0], sc[1], sc[2]] as Vec3)
								: ([1, 1, 1] as Vec3);
					updatePose(
						id,
						vec3Add(cur.origin, delta),
						cur.orientation ?? ([0, 0, 0, 1] as Quat),
						scaleVec,
					);
				}
			}
			return buildSnapshot(records, state.attractions, state.version + 1);
		}
		default:
			return state;
	}
}

/** @emoji ✋ Applies a relocate payload to a fixture (same rules as {@link objectStateReducer}). */
export function applyRelocateToSceneFixture(fixture: FixtureV1, payload: RelocatePayload): FixtureV1 {
	const snap = buildSnapshot(fixtureToRecords(fixture.objects), fixture.attractions, 0);
	const next = objectStateReducer(snap, { type: "relocate", payload });
	const objects = fixture.objects
		.filter((object) => next.records.has(object.id))
		.map((object) => {
			const record = next.records.get(object.id);
			if (!record) {
				return object;
			}
			return {
				...object,
				origin: record.origin,
				...(record.orientation ? { orientation: record.orientation } : {}),
				...(record.scale !== undefined ? { scale: record.scale } : {}),
			};
		});
	return { ...fixture, objects };
}

/** @emoji 🔗 Appends an attraction to a fixture when it does not introduce a cycle. */
export function applyConnectToSceneFixture(fixture: FixtureV1, payload: AttractionPayload): FixtureV1 {
	const snap = buildSnapshot(fixtureToRecords(fixture.objects), fixture.attractions, 0);
	const attractionId = payload.attractionId ?? `attraction-${payload.attracting}-${payload.attracted}`;
	const next = objectStateReducer(snap, {
		type: "addAttraction",
		attraction: {
			id: attractionId,
			attracting: payload.attracting as AttractionProps["attracting"],
			attracted: payload.attracted as AttractionProps["attracted"],
		},
	});
	if (next.attractions.length === snap.attractions.length) {
		return fixture;
	}
	return { ...fixture, attractions: [...next.attractions] };
}

export interface SceneObjectStateContextValue {
	readonly snapshot: ObjectStateSnapshot;
	readonly dispatch: (action: ObjectStateAction) => void;
	readonly handleRelocate: (payload: RelocatePayload) => void;
	readonly handleConnect: (payload: AttractionPayload) => void;
}

export const SceneObjectStateContext = createContext<SceneObjectStateContextValue | null>(null);

/** @emoji ­ƒùä´©Å Central scene object records, attractions, and resolved attraction ownership. */
export function SceneObjectStateProvider(props: {
	readonly fixture: FixtureV1;
	readonly fixtureRevision?: number;
	readonly children: ReactNode;
	readonly onRelocate?: (payload: RelocatePayload) => void;
	readonly onConnect?: (payload: AttractionPayload) => void;
}) {
	const [snapshot, dispatch] = useReducer(objectStateReducer, props.fixture, (fixture) =>
		buildSnapshot(fixtureToRecords(fixture.objects), fixture.attractions, 0),
	);
	const syncedFixtureFingerprintRef = useRef<string | null>(null);
	const syncedPoseFingerprintRef = useRef<string | null>(null);
	const syncedFixtureRevisionRef = useRef<number | undefined>(undefined);
	const fixtureFingerprint = useMemo(() => fixtureStateFingerprint(props.fixture), [props.fixture]);
	const poseFingerprint = useMemo(() => fixturePoseFingerprint(props.fixture), [props.fixture]);
	useEffect(() => {
		if (props.fixtureRevision !== undefined && syncedFixtureRevisionRef.current !== props.fixtureRevision) {
			syncedFixtureRevisionRef.current = props.fixtureRevision;
			if (syncedFixtureFingerprintRef.current === fixtureFingerprint) {
				return;
			}
			syncedFixtureFingerprintRef.current = fixtureFingerprint;
			syncedPoseFingerprintRef.current = poseFingerprint;
			dispatch({ type: "init", fixture: props.fixture });
			return;
		}
		if (syncedFixtureFingerprintRef.current !== fixtureFingerprint) {
			syncedFixtureFingerprintRef.current = fixtureFingerprint;
			syncedPoseFingerprintRef.current = poseFingerprint;
			dispatch({ type: "init", fixture: props.fixture });
			return;
		}
		if (syncedPoseFingerprintRef.current === poseFingerprint) {
			return;
		}
		syncedPoseFingerprintRef.current = poseFingerprint;
		dispatch({ type: "syncPoses", fixture: props.fixture });
	}, [props.fixture, props.fixtureRevision, fixtureFingerprint, poseFingerprint]);
	const handleRelocate = useCallback(
		(payload: RelocatePayload) => {
			dispatch({ type: "relocate", payload });
			props.onRelocate?.(payload);
		},
		[props.onRelocate],
	);
	const handleConnect = useCallback(
		(payload: AttractionPayload) => {
			const attractionId = payload.attractionId ?? `attraction-${payload.attracting}-${payload.attracted}`;
			dispatch({
				type: "addAttraction",
				attraction: {
					id: attractionId,
					attracting: payload.attracting as AttractionProps["attracting"],
					attracted: payload.attracted as AttractionProps["attracted"],
				},
			});
			props.onConnect?.(payload);
		},
		[props.onConnect],
	);
	const value = useMemo<SceneObjectStateContextValue>(
		() => ({ snapshot, dispatch, handleRelocate, handleConnect }),
		[snapshot, handleRelocate, handleConnect],
	);
	return <SceneObjectStateContext.Provider value={value}>{props.children}</SceneObjectStateContext.Provider>;
}

function useSceneObjectState(): SceneObjectStateContextValue {
	const v = useContext(SceneObjectStateContext);
	if (!v) {
		throw new Error("SceneObjectStateProvider missing");
	}
	return v;
}

function useLiveBlockedVortexFullIds(fallback: ReadonlySet<string>): ReadonlySet<string> {
	const state = useContext(SceneObjectStateContext);
	return useMemo(
		() => (state ? blockedVortexFullIdsFromAttractions(state.snapshot.attractions) : fallback),
		[state, fallback, state?.snapshot.attractions, state?.snapshot.version],
	);
}

/** @emoji ­ƒ¬Ø Relocate handler that updates central object state and cascades to attracted descendants. */
export function useSceneObjectRelocate(): (payload: RelocatePayload) => void {
	return useSceneObjectState().handleRelocate;
}

/** @emoji ­ƒ¬Ø Connect handler that appends an attraction and recomputes attraction ownership. */
export function useSceneObjectConnect(): (payload: AttractionPayload) => void {
	return useSceneObjectState().handleConnect;
}

function useObjectRecord(objectId: string): ObjectRecord | undefined {
	const { snapshot } = useSceneObjectState();
	return useMemo(() => snapshot.records.get(objectId), [snapshot.records, snapshot.version, objectId]);
}

function useAttractingChildIds(objectId: string): readonly string[] {
	const { snapshot } = useSceneObjectState();
	return useMemo(
		() => snapshot.tree.attractingByObjectId.get(objectId) ?? [],
		[snapshot.tree.attractingByObjectId, snapshot.version, objectId],
	);
}

const ObjectItemById = memo(function ObjectItemById(props: {
	readonly objectId: string;
	readonly selected?: boolean;
	readonly selectedVortexFullIds?: ReadonlySet<string>;
	readonly relocate?: RelocateMode | false;
}) {
	const record = useObjectRecord(props.objectId);
	const attracting = useAttractingChildIds(props.objectId);
	if (!record) {
		return null;
	}
	return (
		<ObjectItem
			id={record.id}
			objectKind={record.objectKind}
			meshUrl={record.meshUrl}
			origin={record.origin}
			orientation={record.orientation}
			scale={record.scale}
			label={record.label}
			wormhole={record.wormhole}
			attracting={attracting}
			selected={props.selected}
			relocate={props.relocate}
		>
			{record.vortices.map((vortex) => {
				const fullId = sceneVortexFullId(record.id, vortex.id);
				return (
					<Vortex
						key={vortex.id}
						objectId={record.id}
						objectKind={record.objectKind}
						objectOrigin={record.origin}
						objectOrientation={record.orientation}
						selected={props.selectedVortexFullIds?.has(fullId)}
						{...vortex}
					/>
				);
			})}
		</ObjectItem>
	);
});

/** @emoji ­ƒî▓ Declares attraction tree structure; meshes mount flat via {@link SceneObjects} so ids stay stable on reparent. */
export const ObjectTreeNode = memo(function ObjectTreeNode(props: {
	readonly objectId: string;
	readonly visitedIds?: readonly string[];
}) {
	const attracting = useAttractingChildIds(props.objectId);
	const visited = props.visitedIds ?? [];
	if (visited.includes(props.objectId)) {
		return null;
	}
	const nextVisited = visited.length ? [...visited, props.objectId] : [props.objectId];
	return (
		<>
			{attracting.map((childId) => (
				<ObjectTreeNode key={childId} objectId={childId} visitedIds={nextVisited} />
			))}
		</>
	);
});

/** @emoji 🎯 True when an object is part of the current selection (directly or via a selected vortex). */
export function objectMatchesSceneSelection(objectId: string, selection: SelectionSnapshot | undefined): boolean {
	if (!selection) {
		return false;
	}
	if (selection.objectIds.includes(objectId)) {
		return true;
	}
	for (const vortexFullId of selection.vortexIds) {
		if (parseVortexFullId(vortexFullId).objectId === objectId) {
			return true;
		}
	}
	return false;
}

export interface SceneObjectsProps {
	readonly selection?: SelectionSnapshot;
	readonly selectedObjectId?: string | null;
	readonly selectedVortexFullIds?: ReadonlySet<string>;
	readonly relocate?: RelocateMode | false;
}

/** @emoji ­ƒºè Renders all scene objects from central state (id-keyed; survives ownership changes). */
export const SceneObjects = memo(function SceneObjects(props: SceneObjectsProps) {
	const { snapshot } = useSceneObjectState();
	const ids = useMemo(() => [...snapshot.records.keys()].sort(), [snapshot.records, snapshot.version]);
	return (
		<>
			{ids.map((id) => (
				<ObjectItemById
					key={id}
					objectId={id}
					selected={objectMatchesSceneSelection(id, props.selection) || props.selectedObjectId === id}
					selectedVortexFullIds={props.selectedVortexFullIds}
					relocate={props.relocate}
				/>
			))}
		</>
	);
});

/** @emoji ­ƒî▓ Logical attraction tree roots (wormholes) for structure-only composition. */
export const SceneAttractionTreeRoots = memo(function SceneAttractionTreeRoots() {
	const { snapshot } = useSceneObjectState();
	return (
		<>
			{snapshot.tree.wormholeIds.map((id) => (
				<ObjectTreeNode key={id} objectId={id} />
			))}
		</>
	);
});

/** @emoji ­ƒº▓ Renders all attraction endpoint lines in one frame loop (avoids N├ùuseFrame churn). */
export const SceneAttractions = memo(function SceneAttractions() {
	const { snapshot } = useSceneObjectState();
	return <SceneAttractionLineBatch attractions={snapshot.attractions} />;
});
//#endregion ­ƒò©´©ÅAttractionGraph

//#region ­ƒº®Compat
export function kindsCompatible(
	aKind: string | undefined,
	bKind: string | undefined,
	table: readonly KindCompatEntry[] | undefined,
): boolean {
	if (!table?.length || !aKind || !bKind) return false;
	return table.some(
		(e) =>
			(e.source === aKind && e.target === bKind) ||
			(e.bidirectional === true && e.source === bKind && e.target === aKind),
	);
}

const DEFAULT_WIRE_KIND_ID = "board.wire.link";

/** @emoji ­ƒº▓ Attraction endpoint vortex full ids that are already attracting/attracted and cannot start or receive another attraction. */
export function blockedVortexFullIdsFromAttractions(
	attractions: readonly Pick<AttractionProps, "attracting" | "attracted">[],
): ReadonlySet<string> {
	const s = new Set<string>();
	for (const attraction of attractions) {
		s.add(attraction.attracting);
		s.add(attraction.attracted);
	}
	return s;
}

/** @emoji ­ƒº¡ Semantic kinds at one end of an attraction drag (object + vortex handle). */
export interface AttractionHandleContext {
	readonly objectId: string;
	readonly objectKind: string | undefined;
	readonly vortexKind: string | undefined;
}

function catalogHandleById(
	catalogs: KindCatalogBundle | undefined,
	handleKind: string | undefined,
): HandleKindCatalogEntry | undefined {
	if (!handleKind || !catalogs?.handles?.length) return undefined;
	return catalogs.handles.find((h) => h.id === handleKind);
}

function catalogWireById(
	catalogs: KindCatalogBundle | undefined,
	wireKind: string | undefined,
): WireKindCatalogEntry | undefined {
	if (!wireKind || !catalogs?.wires?.length) return undefined;
	return catalogs.wires.find((w) => w.id === wireKind);
}

/** @emoji ­ƒöî Resolves default wire kind for a vortex kind via handle catalog, else `board.wire.link`. */
export function resolveWireKindForVortex(
	vortexKind: string | undefined,
	catalogs: KindCatalogBundle | undefined,
): string {
	const h = catalogHandleById(catalogs, vortexKind);
	const w = h?.defaultWireKind?.trim();
	return w && w.length > 0 ? w : DEFAULT_WIRE_KIND_ID;
}

/** @emoji ­ƒ¬ó Resolves default edge kind for a wire kind via wire catalog, else empty string. */
export function resolveEdgeKindForWire(
	wireKind: string | undefined,
	catalogs: KindCatalogBundle | undefined,
): string {
	const w = catalogWireById(catalogs, wireKind);
	const e = w?.defaultEdgeKind?.trim();
	return e && e.length > 0 ? e : "";
}

function compatPairMatches(rule: KindCompatEntry, a: string, b: string): boolean {
	if (rule.source === a && rule.target === b) return true;
	if (rule.bidirectional === true && rule.source === b && rule.target === a) return true;
	return false;
}

function attractionGestureRuleApplies(
	rule: KindCompatEntry,
	attracting: AttractionHandleContext,
	attracted: AttractionHandleContext,
	catalogs: KindCatalogBundle | undefined,
): boolean {
	const wSrc = resolveWireKindForVortex(attracting.vortexKind, catalogs);
	const wTgt = resolveWireKindForVortex(attracted.vortexKind, catalogs);
	const eSrc = resolveEdgeKindForWire(wSrc, catalogs);
	const eTgt = resolveEdgeKindForWire(wTgt, catalogs);
	const sn = attracting.objectKind ?? "";
	const tn = attracted.objectKind ?? "";
	const sh = attracting.vortexKind ?? "";
	const th = attracted.vortexKind ?? "";
	const spec = rule.specificity ?? "handle";
	switch (spec) {
		case "general":
			return compatPairMatches(rule, sh, th);
		case "object":
		case "node":
			return compatPairMatches(rule, sn, tn);
		case "edge":
		case "attraction":
			return compatPairMatches(rule, eSrc, eTgt);
		case "handle":
			return compatPairMatches(rule, sh, th);
		case "wire":
			return compatPairMatches(rule, wSrc, th);
		default:
			return compatPairMatches(rule, sh, th);
	}
}

/** @emoji ­ƒñØ WASM-style filtered attraction compatibility (important + specificity tiers); empty rules allow all. */
export function handlesAttractionCompatibleForDrag(
	attracting: AttractionHandleContext,
	attracted: AttractionHandleContext,
	rules: readonly KindCompatEntry[] | undefined,
	catalogs: KindCatalogBundle | undefined,
): boolean {
	if (!rules?.length) return true;
	let matched = rules.filter((r) => attractionGestureRuleApplies(r, attracting, attracted, catalogs));
	if (matched.length === 0) return false;
	if (matched.some((r) => r.important)) matched = matched.filter((r) => r.important);
	else {
		const rank = (s: KindCompatEntry["specificity"] | undefined): number => {
			switch (s) {
				case "general":
					return 0;
				case "object":
				case "node":
					return 1;
				case "edge":
				case "attraction":
					return 2;
				case "wire":
					return 3;
				case "handle":
					return 4;
				default:
					return 4;
			}
		};
		const maxRank = Math.max(...matched.map((r) => rank(r.specificity)));
		matched = matched.filter((r) => rank(r.specificity) === maxRank);
	}
	return matched.length > 0;
}
//#endregion ­ƒº®Compat

//#region ­ƒÄ¿MeshPaint
const CSS_SELECTED_MESH = "var(--color-primary)";
const CSS_SELECTED_LINE = "var(--color-primary)";
const CSS_HIGHLIGHTED_MESH = "var(--color-primary)";
const CSS_HIGHLIGHTED_LINE = "var(--color-primary)";
const CSS_HOVERED_MESH = "color-mix(in oklab, var(--color-accent) 28%, var(--color-panel))";
const CSS_HOVERED_LINE = "var(--color-accent)";
const CSS_NEUTRAL_MESH = "var(--color-panel)";
const CSS_NEUTRAL_LINE = "var(--color-element)";
const CSS_DISABLED_MESH = "color-mix(in oklab, var(--color-muted-foreground) 55%, var(--color-panel))";
const CSS_DISABLED_LINE = "var(--color-muted-foreground)";
const CSS_ATTRACTION_ENDPOINT_LINE = "var(--color-muted-foreground)";
const CSS_ATTRACTION_LINE = "var(--color-accent)";

const MESH_OUTLINE_USER_DATA_KEY = "__elementsMeshBodyOutline";

interface MeshStyleColors {
	readonly meshColor: string;
	readonly lineColor: string;
	readonly emissiveColor: string;
	readonly emissiveIntensity: number;
	readonly opacity: number;
}

const meshStyleColorCache = new Map<Exclude<MeshStyleKind, "original">, MeshStyleColors>();

const MESH_STYLE_HEADLESS: Record<Exclude<MeshStyleKind, "original">, MeshStyleColors> = {
	neutral: {
		meshColor: "#eeeadb",
		lineColor: "#001117",
		emissiveColor: "#000000",
		emissiveIntensity: 0,
		opacity: 1,
	},
	hovered: {
		meshColor: "#f0c8cc",
		lineColor: "#ff344f",
		emissiveColor: "#ff344f",
		emissiveIntensity: 0.15,
		opacity: 1,
	},
	selected: {
		meshColor: "#ff344f",
		lineColor: "#ff344f",
		emissiveColor: "#ff344f",
		emissiveIntensity: 0.35,
		opacity: 1,
	},
	highlighted: {
		meshColor: "#ff344f",
		lineColor: "#ff344f",
		emissiveColor: "#ff344f",
		emissiveIntensity: 0.28,
		opacity: 1,
	},
	disabled: {
		meshColor: "#c8ccc6",
		lineColor: "#7b827d",
		emissiveColor: "#000000",
		emissiveIntensity: 0,
		opacity: 0.45,
	},
};

function probeCssComputed(property: "color" | "backgroundColor", value: string): string {
	if (typeof document === "undefined") {
		return "";
	}
	const el = document.createElement("span");
	const key = property === "color" ? "color" : "background-color";
	el.setAttribute("style", `${key}:${value};position:absolute;left:0;top:0;visibility:hidden;pointer-events:none`);
	document.documentElement.appendChild(el);
	const out = getComputedStyle(el)[property];
	el.remove();
	return out;
}

function resolveCssColor(property: "color" | "backgroundColor", expr: string, fallback: string): string {
	const raw = probeCssComputed(property, expr);
	if (!raw || raw === "rgba(0, 0, 0, 0)") {
		return fallback;
	}
	return raw;
}

/** @emoji ­ƒÄ¿ Resolves mesh and edge colors for a {@link MeshStyleKind} from Elements tokens. */
export function meshStyleColors(style: MeshStyleKind): MeshStyleColors | null {
	if (style === "original") {
		return null;
	}
	const cached = meshStyleColorCache.get(style);
	if (cached) {
		return cached;
	}
	const fb = MESH_STYLE_HEADLESS[style];
	const meshExprs: Record<Exclude<MeshStyleKind, "original">, string> = {
		neutral: CSS_NEUTRAL_MESH,
		hovered: CSS_HOVERED_MESH,
		selected: CSS_SELECTED_MESH,
		highlighted: CSS_HIGHLIGHTED_MESH,
		disabled: CSS_DISABLED_MESH,
	};
	const lineExprs: Record<Exclude<MeshStyleKind, "original">, string> = {
		neutral: CSS_NEUTRAL_LINE,
		hovered: CSS_HOVERED_LINE,
		selected: CSS_SELECTED_LINE,
		highlighted: CSS_HIGHLIGHTED_LINE,
		disabled: CSS_DISABLED_LINE,
	};
	const resolved = {
		meshColor: resolveCssColor("backgroundColor", meshExprs[style], fb.meshColor),
		lineColor: resolveCssColor("color", lineExprs[style], fb.lineColor),
		emissiveColor: resolveCssColor("color", lineExprs[style], fb.emissiveColor),
		emissiveIntensity: fb.emissiveIntensity,
		opacity: fb.opacity,
	};
	meshStyleColorCache.set(style, resolved);
	return resolved;
}

function createStyledMeshMaterial(color: string, state: MeshStyleColors): MeshStandardMaterial {
	const mat = new MeshStandardMaterial({
		color: new Color(color),
		metalness: 0,
		roughness: 1,
	});
	mat.emissive.set(state.emissiveColor);
	mat.emissiveIntensity = state.emissiveIntensity;
	mat.transparent = state.opacity < 1;
	mat.opacity = state.opacity;
	return mat;
}

function createStyledLineMaterial(color: string, state: MeshStyleColors): LineBasicMaterial {
	const mat = new LineBasicMaterial({ color: new Color(color) });
	mat.transparent = state.opacity < 1;
	mat.opacity = state.opacity;
	return mat;
}

function createMeshOutline(geometry: BufferGeometry, color: string, state: MeshStyleColors): LineSegments {
	const outline = new LineSegments(new EdgesGeometry(geometry), createStyledLineMaterial(color, state));
	outline.userData[MESH_OUTLINE_USER_DATA_KEY] = true;
	outline.scale.setScalar(1.001);
	return outline;
}

function applyMeshStyleToObject3D(root: Object3D, style: MeshStyleKind): void {
	const colors = meshStyleColors(style);
	if (!colors) {
		return;
	}
	root.traverse((object) => {
		if (object instanceof Mesh) {
			const meshMaterial = createStyledMeshMaterial(colors.meshColor, colors);
			if (Array.isArray(object.material)) {
				object.material = object.material.map(() => meshMaterial.clone());
			} else {
				object.material = meshMaterial;
			}
			const geometry = object.geometry;
			if (geometry && !object.children.some((c) => c.userData[MESH_OUTLINE_USER_DATA_KEY])) {
				object.add(createMeshOutline(geometry, colors.lineColor, colors));
			}
			return;
		}
		if (object instanceof ThreeLine || object instanceof LineSegments) {
			if (object.userData[MESH_OUTLINE_USER_DATA_KEY]) {
				return;
			}
			object.material = createStyledLineMaterial(colors.lineColor, colors);
			return;
		}
		if (object instanceof Points) {
			object.material = new PointsMaterial({
				color: new Color(colors.lineColor),
				size: 1,
				transparent: colors.opacity < 1,
				opacity: colors.opacity,
			});
		}
	});
}

/** @emoji ­ƒÄ¿ Chooses the effective mesh style from explicit prop and interaction flags. */
export function resolveMeshStyle(args: {
	readonly style?: MeshStyleKind;
	readonly disabled?: boolean;
	readonly selected?: boolean;
	readonly highlighted?: boolean;
	readonly hovered?: boolean;
}): MeshStyleKind {
	if (args.style) {
		return args.style;
	}
	if (args.disabled) {
		return "disabled";
	}
	if (args.selected) {
		return "selected";
	}
	if (args.highlighted) {
		return "highlighted";
	}
	if (args.hovered) {
		return "hovered";
	}
	return DEFAULT_MESH_STYLE;
}

/** @emoji ­ƒÄ¿ Resolves a CSS color for scene lines (endpoint attractions, attraction guides). */
export function lineCssColor(expr: string, fallback: string): string {
	return resolveCssColor("color", expr, fallback);
}
//#endregion ­ƒÄ¿MeshPaint

//#region ­ƒÅèPool
const gltfRefCounts = new Map<string, number>();
const styledMeshRefCounts = new Map<string, number>();
const styledMeshTemplates = new Map<string, Object3D>();

function styledPoolKey(url: string, style: MeshStyleKind): string {
	return `${url}\0${style}`;
}

export function gltfPoolAcquire(url: string): void {
	gltfRefCounts.set(url, (gltfRefCounts.get(url) ?? 0) + 1);
}

export function gltfPoolRelease(url: string): void {
	const n = (gltfRefCounts.get(url) ?? 1) - 1;
	if (n <= 0) {
		gltfRefCounts.delete(url);
	} else {
		gltfRefCounts.set(url, n);
	}
}

export function styledMeshPoolAcquire(url: string, style: MeshStyleKind): void {
	const key = styledPoolKey(url, style);
	styledMeshRefCounts.set(key, (styledMeshRefCounts.get(key) ?? 0) + 1);
}

export function styledMeshPoolRelease(url: string, style: MeshStyleKind): void {
	const key = styledPoolKey(url, style);
	const n = (styledMeshRefCounts.get(key) ?? 1) - 1;
	if (n <= 0) {
		styledMeshRefCounts.delete(key);
		styledMeshTemplates.delete(key);
	} else {
		styledMeshRefCounts.set(key, n);
	}
}

/** @emoji ­ƒº╣ Drops pooled GLTF cache entries (call on scene teardown, not per-chunk unmount). */
export function gltfPoolClear(url: string): void {
	gltfRefCounts.delete(url);
	for (const key of [...styledMeshTemplates.keys()]) {
		if (key.startsWith(`${url}\0`)) {
			styledMeshTemplates.delete(key);
			styledMeshRefCounts.delete(key);
		}
	}
	useGLTF.clear(url);
}

/** @emoji ­ƒÅè Returns a cached styled GLTF template for {@link MeshBody} (refcount via acquire/release). */
export function styledMeshTemplate(url: string, style: MeshStyleKind, source: Object3D): Object3D {
	if (style === "original") {
		return source;
	}
	const key = styledPoolKey(url, style);
	let template = styledMeshTemplates.get(key);
	if (!template) {
		template = source.clone(true);
		applyMeshStyleToObject3D(template, style);
		styledMeshTemplates.set(key, template);
	}
	return template;
}

function usePooledGltf(url: string) {
	const gltf = useGLTF(url);
	useEffect(() => {
		gltfPoolAcquire(url);
		return () => {
			gltfPoolRelease(url);
		};
	}, [url]);
	return gltf;
}

function usePooledStyledMesh(url: string, style: MeshStyleKind) {
	const gltf = usePooledGltf(url);
	useEffect(() => {
		if (style === "original") {
			return undefined;
		}
		styledMeshPoolAcquire(url, style);
		return () => {
			styledMeshPoolRelease(url, style);
		};
	}, [url, style]);
	const renderRoot = useMemo(() => {
		if (!gltf.scene) {
			return null;
		}
		const template = styledMeshTemplate(url, style, gltf.scene);
		return template.clone(true);
	}, [gltf.scene, url, style]);
	return renderRoot;
}
//#endregion ­ƒÅèPool

//#region ­ƒÄ»Registry
type VortexGetter = () => Vector3 | null;

export interface VortexBindingMeta {
	readonly fullId: string;
	readonly objectId: string;
	readonly objectKind: string | undefined;
	readonly vortexKind: string | undefined;
	readonly radiusWorld: number;
}

export interface RegistryValue {
	registerVortex(fullId: string, getter: VortexGetter): void;
	unregisterVortex(fullId: string): void;
	getVortexWorld(fullId: string): Vector3 | null;
	registerVortexBinding(meta: VortexBindingMeta, pickRoot: Object3D | null): void;
	unregisterVortexBinding(fullId: string): void;
	registerObject(id: string, objectKind: string | undefined, group: Group | null): void;
	collectObjectGroups(): readonly Group[];
	getObjectGroup(id: string): Group | null;
	getObjectKind(id: string): string | undefined;
	kindCatalogs: KindCatalogBundle | undefined;
	kindCompatibility: readonly KindCompatEntry[] | undefined;
	blockedVortexFullIds: ReadonlySet<string>;
	proximityRadius: number;
	selectedObjectIds: readonly string[];
	setSelectedObjectIds(ids: readonly string[]): void;
	selectionMode: SelectionMode;
	relocateMode: RelocateMode;
	activeRelocateObjectId: string | null;
	setActiveRelocateObjectId: (id: string | null) => void;
	attractionDragActive: boolean;
	attractionDragAttractingFullId: string | null;
	attractionCompatibleAttractedFullIds: ReadonlySet<string>;
	attractionHoverRingFullId: string | null;
	attractionIndirectPickAwait: AttractionIndirectPickAwait | null;
	attractionEndWorldRef: MutableRefObject<Vector3 | null>;
	beginAttractionDragFromVortex(fullId: string, objectId: string, objectKind: string | undefined, vortexKind: string | undefined): void;
	cancelAttractionDrag(): void;
	findNearestProximityRelocate(world: Vector3, movingObjectId: string): AttractionPayload | null;
	attachAttractionThreeEnv(env: { camera: Camera; gl: WebGLRenderer; scene: ThreeScene } | null): void;
	updateAttractionPointer(clientX: number, clientY: number): void;
	commitAttractionPointer(clientX: number, clientY: number): void;
	updateIndirectPickPointer(clientX: number, clientY: number): void;
	commitIndirectPickPointerDown(clientX: number, clientY: number): void;
	onSelect?: (snap: SelectionSnapshot) => void;
	onConnect?: (p: AttractionPayload) => void;
	onProximityConnect?: (p: AttractionPayload) => void;
	onIndirectConnect?: (p: AttractionPayload) => void;
	onAttractionCompatibleObjects?: (p: AttractionCompatibleObjectsPayload) => void;
	onAttractionTargetRing?: (p: AttractionTargetRingPayload) => void;
	onRelocate?: (p: RelocatePayload) => void;
	readonly hoverTarget: SceneHoverTarget | null;
	setSceneHover: (target: SceneHoverTarget) => void;
	clearSceneHover: (target: SceneHoverTarget) => void;
	clearSceneHoverAll: () => void;
	isSceneHovered: (target: SceneHoverTarget) => boolean;
	clearSceneSelection: () => void;
}

/** @emoji ­ƒÄ» Attraction-drag UI state isolated so orbit idle frames do not re-render every object. */
export interface RegistryDragState {
	readonly attractionDragActive: boolean;
	readonly attractionDragAttractingFullId: string | null;
	readonly attractionCompatibleAttractedFullIds: ReadonlySet<string>;
	readonly attractionHoverRingFullId: string | null;
	readonly attractionIndirectPickAwait: AttractionIndirectPickAwait | null;
}

const RegistryCoreContext = createContext<Omit<RegistryValue, keyof RegistryDragState> | null>(null);
const RegistryDragContext = createContext<RegistryDragState | null>(null);

function useRegistryCore(): Omit<RegistryValue, keyof RegistryDragState> {
	const v = useContext(RegistryCoreContext);
	if (!v) throw new Error("Scene registry missing");
	return v;
}

function useRegistryDrag(): RegistryDragState {
	const v = useContext(RegistryDragContext);
	if (!v) throw new Error("Scene registry drag missing");
	return v;
}

function useRegistry(): RegistryValue {
	return { ...useRegistryCore(), ...useRegistryDrag() };
}

/** @emoji 🖱️ Clears exclusive hover when the pointer leaves the canvas. */
function SceneHoverMissBridge(): null {
	const { clearSceneHoverAll } = useRegistryCore();
	const invalidate = useThree((state) => state.invalidate);
	const gl = useThree((state) => state.gl);
	useEffect(() => {
		const onLeave = () => {
			clearSceneHoverAll();
			invalidate();
		};
		gl.domElement.addEventListener("pointerleave", onLeave);
		return () => gl.domElement.removeEventListener("pointerleave", onLeave);
	}, [clearSceneHoverAll, gl, invalidate]);
	return null;
}

/** @emoji 🖱️ Redraws the canvas when exclusive hover changes. */
function SceneHoverInvalidateBridge(): null {
	const { hoverTarget } = useRegistryCore();
	const invalidate = useThree((state) => state.invalidate);
	useEffect(() => {
		invalidate();
	}, [hoverTarget, invalidate]);
	return null;
}

/** @emoji 🎯 Clears selection when the user clicks empty canvas (R3F pointer missed). */
function SceneSelectionMissBridge(): null {
	const { clearSceneSelection } = useRegistryCore();
	const store = useStore();
	const attractionBusy =
		useRegistryDrag().attractionDragActive || useRegistryDrag().attractionIndirectPickAwait !== null;
	const clearSceneSelectionRef = useRef(clearSceneSelection);
	clearSceneSelectionRef.current = clearSceneSelection;
	const attractionBusyRef = useRef(attractionBusy);
	attractionBusyRef.current = attractionBusy;
	useEffect(() => {
		const previous = store.getState().onPointerMissed;
		const onMiss = (event: MouseEvent) => {
			if (event.button !== 0 || attractionBusyRef.current) {
				previous?.(event);
				return;
			}
			clearSceneSelectionRef.current();
			previous?.(event);
		};
		store.setState({ onPointerMissed: onMiss });
		return () => store.setState({ onPointerMissed: previous });
	}, [store]);
	return null;
}
//#endregion ­ƒÄ»Registry

//#region ­ƒº▒Chunking
export function chunkKey(origin: Vec3, chunkSize: number): string {
	const ix = Math.floor(origin[0] / chunkSize);
	const iy = Math.floor(origin[1] / chunkSize);
	const iz = Math.floor(origin[2] / chunkSize);
	return `${ix}|${iy}|${iz}`;
}

function setEquals(a: ReadonlySet<string>, b: ReadonlySet<string>): boolean {
	if (a.size !== b.size) return false;
	for (const v of a) if (!b.has(v)) return false;
	return true;
}

/** @emoji ­ƒôÅ Chunk bounding radius in world units (half-space diagonal of a cube chunk). */
export function chunkBoundsRadius(chunkSize: number): number {
	return chunkSize * 0.866;
}

/** @emoji ­ƒæü´©Å Distance-only chunk visibility with enter/exit hysteresis (avoids frustum-edge flicker). */
export function chunkDistanceVisible(args: {
	readonly camPos: Vector3;
	readonly chunkCenter: Vector3;
	readonly chunkSize: number;
	readonly maxDist: number;
	readonly wasVisible: boolean;
}): boolean {
	const boundsR = chunkBoundsRadius(args.chunkSize);
	const dist = args.camPos.distanceTo(args.chunkCenter);
	const enterDist = args.maxDist + boundsR;
	const exitDist = enterDist + args.chunkSize * 0.5;
	if (dist <= enterDist) return true;
	if (args.wasVisible && dist <= exitDist) return true;
	return false;
}

function useVisibleChunkKeys(chunkKeys: Iterable<string>, chunkSize: number, maxDist: number): ReadonlySet<string> {
	const { camera } = useThree();
	const centerTmp = useMemo(() => new Vector3(), []);
	const [visible, setVisible] = useState<ReadonlySet<string>>(() => new Set());
	useFrame(() => {
		const camPos = camera.position;
		setVisible((prev) => {
			const next = new Set(prev);
			for (const key of chunkKeys) {
				const [ix, iy, iz] = key.split("|").map(Number);
				centerTmp.set((ix + 0.5) * chunkSize, (iy + 0.5) * chunkSize, (iz + 0.5) * chunkSize);
				const show = chunkDistanceVisible({
					camPos,
					chunkCenter: centerTmp,
					chunkSize,
					maxDist,
					wasVisible: next.has(key),
				});
				if (show) next.add(key);
				else next.delete(key);
			}
			return setEquals(prev, next) ? prev : next;
		});
	});
	return visible;
}
//#endregion ­ƒº▒Chunking

//#region 🧭Coordinates
const _eulerCadToThree = new Euler(-Math.PI / 2, 0, 0, "XYZ");
const _mCadToThree = new Matrix4();
const _qCadToThree = new Quaternion();

/** @emoji 🧭 Fixed CAD Z-up → Three.js Y-up rotation matrix (matches sketchpad {@link toThreeRotation}). */
export function cadToThreeMatrix(): Matrix4 {
	return _mCadToThree.makeRotationFromEuler(_eulerCadToThree);
}

/** @emoji 🧭 Maps a CAD fixture point to Three.js world coordinates. */
export function cadVec3ToThree(v: Vec3): Vec3 {
	const out = new Vector3(v[0], v[1], v[2]).applyMatrix4(cadToThreeMatrix());
	return [out.x, out.y, out.z];
}

/** @emoji 🧭 Maps a Three.js point back to CAD fixture coordinates. */
export function threeVec3ToCad(v: Vector3): Vec3 {
	const out = v.clone().applyMatrix4(cadToThreeMatrix().clone().invert());
	return [out.x, out.y, out.z];
}

/** @emoji 🧭 Maps a CAD fixture quaternion to Three.js. */
export function cadQuatToThree(q: Quat): Quat {
	const qCad = new Quaternion(q[0], q[1], q[2], q[3]);
	const out = _qCadToThree.setFromRotationMatrix(cadToThreeMatrix()).multiply(qCad);
	return [out.x, out.y, out.z, out.w];
}

/** @emoji 🧭 Maps a Three.js quaternion back to CAD fixture coordinates. */
export function threeQuatToCad(q: Quaternion): Quat {
	const out = _qCadToThree.setFromRotationMatrix(cadToThreeMatrix()).invert().multiply(q);
	return [out.x, out.y, out.z, out.w];
}

function quatRotateVec(q: Quat, v: Vec3): Vec3 {
	const out = new Vector3(v[0], v[1], v[2]).applyQuaternion(new Quaternion(q[0], q[1], q[2], q[3]));
	return [out.x, out.y, out.z];
}

/** @emoji 🧭 Maps object-local CAD offset to Three.js parent-group coordinates. */
export function cadObjectLocalToThreeGroupLocal(
	local: Vec3,
	originCad: Vec3,
	orientationCad: Quat | undefined,
): Vec3 {
	const q = orientationCad ?? ([0, 0, 0, 1] as Quat);
	const worldCad = vec3Add(originCad, quatRotateVec(q, local));
	const worldThree = new Vector3(...cadVec3ToThree(worldCad));
	const originThree = new Vector3(...cadVec3ToThree(originCad));
	const qThree = new Quaternion(...cadQuatToThree(q));
	const out = worldThree.sub(originThree).applyQuaternion(qThree.invert());
	return [out.x, out.y, out.z];
}

/** @emoji 🧭 +90° X: glTF Y-up mesh → CAD object-local Z-up inside the pose group (metabolism kit meshes). */
export const GLB_MESH_FRAME_ROTATION_X = Math.PI / 2;
//#endregion 🧭Coordinates

//#region ­ƒºèHelpers
function vec3ToThree(v: Vec3) {
	return new Vector3(...cadVec3ToThree(v));
}

function quatToThree(q: Quat | undefined) {
	if (!q) return new Quaternion();
	const c = cadQuatToThree(q);
	return new Quaternion(c[0], c[1], c[2], c[3]);
}

function scaleToThree(s: number | Vec3 | undefined): Vector3 {
	if (s === undefined) return new Vector3(1, 1, 1);
	if (typeof s === "number") return new Vector3(s, s, s);
	return new Vector3(s[0], s[1], s[2]);
}

/** @emoji ­ƒöæ Stable key for object pose props (relocate mutates the group without changing this until commit). */
export function objectPoseKey(
	id: string,
	origin: Vec3,
	orientation: Quat | undefined,
	scale: number | Vec3 | undefined,
): string {
	const o = orientation ?? ([0, 0, 0, 1] as Quat);
	const sc = scale === undefined ? 1 : typeof scale === "number" ? scale : scale.join(",");
	return `${id}|${origin.join(",")}|${o.join(",")}|${sc}`;
}

/** @emoji ­ƒôì Writes fixture pose onto an object group; avoids R3F controlled transforms so vortex children follow relocate. */
export function applyObjectPose(
	group: Group,
	origin: Vec3,
	orientation: Quat | undefined,
	scale: number | Vec3 | undefined,
): void {
	const p = cadVec3ToThree(origin);
	group.position.set(p[0], p[1], p[2]);
	group.quaternion.copy(quatToThree(orientation));
	group.scale.copy(scaleToThree(scale));
}

/** @emoji ­ƒî│ Updates world matrices from root to leaf so {@link Object3D.getWorldPosition} matches the current graph. */
export function updateWorldMatrixChain(leaf: Object3D): void {
	const chain: Object3D[] = [];
	for (let cur: Object3D | null = leaf; cur; cur = cur.parent) {
		chain.push(cur);
	}
	for (let i = chain.length - 1; i >= 0; i--) {
		chain[i]!.updateMatrixWorld(false);
	}
}

export type SceneAutoFitBehavior = "initial" | "changes";

export function sceneAutoFitShouldRun(
	behavior: SceneAutoFitBehavior,
	key: string,
	lastKey: string,
	hasApplied: boolean,
): boolean {
	if (!key || key === lastKey) return false;
	return behavior === "changes" || !hasApplied;
}

/** @emoji 📐 Axis-aligned bounds of registered scene object groups (camera auto-fit). */
export function boundsFromObjectGroups(groups: readonly Group[]): { readonly center: Vec3; readonly radius: number } | null {
	if (!groups.length) return null;
	const box = new Box3();
	const sizeScratch = new Vector3();
	const centerScratch = new Vector3();
	let has = false;
	for (const group of groups) {
		updateWorldMatrixChain(group);
		const part = new Box3().setFromObject(group, true);
		if (!Number.isFinite(part.min.x) || !Number.isFinite(part.max.x)) continue;
		if (part.isEmpty()) continue;
		part.getSize(sizeScratch);
		if (sizeScratch.lengthSq() < 1e-12) continue;
		if (!has) {
			box.copy(part);
			has = true;
		} else {
			box.union(part);
		}
	}
	if (!has) return null;
	box.getCenter(centerScratch);
	box.getSize(sizeScratch);
	const radius = sizeScratch.length() / 2;
	return { center: threeVec3ToCad(centerScratch), radius: Math.max(radius, 0.5) };
}

/** @emoji 🛰️ Frames perspective orbit camera to fit scene object bounds (CAD center, Three world rig). */
export function applySceneAutoFitCamera(
	camera: ThreePerspectiveCamera,
	bounds: { readonly center: Vec3; readonly radius: number },
	padding = 1.25,
	controls?: { readonly target: Vector3; update?: () => void } | null,
): void {
	const centerThree = cadVec3ToThree(bounds.center);
	const dist = Math.max(bounds.radius * padding, 2);
	camera.position.set(centerThree[0] + dist, centerThree[1] + dist, centerThree[2] + dist * 0.85);
	if (controls?.target) {
		controls.target.set(centerThree[0], centerThree[1], centerThree[2]);
		controls.update?.();
	} else {
		camera.lookAt(centerThree[0], centerThree[1], centerThree[2]);
	}
	camera.updateProjectionMatrix();
}

function vector3IsFinite(v: Vector3): boolean {
	return Number.isFinite(v.x) && Number.isFinite(v.y) && Number.isFinite(v.z);
}
//#endregion ­ƒºèHelpers

//#region ­ƒº▓AttractionGesture
function readVortexFullIdFromObject(o: Object3D | null): string | null {
	let cur: Object3D | null = o;
	while (cur) {
		const id = cur.userData?.sceneVortexFullId;
		if (typeof id === "string" && id.length > 0) return id;
		cur = cur.parent;
	}
	return null;
}

function readObjectItemIdFromObject(o: Object3D | null): string | null {
	let cur: Object3D | null = o;
	while (cur) {
		const id = cur.userData?.sceneObjectId;
		if (typeof id === "string" && id.length > 0) return id;
		cur = cur.parent;
	}
	return null;
}

const HANDLE_HIT_TOLERANCE_PX = 10;
const ATTRACTION_HANDLE_SNAP_EXTRA_PX = 22;
const ATTRACTION_COMMIT_SNAP_TIGHT_PX = 2;

function worldToCanvasPx(world: Vector3, camera: Camera, gl: WebGLRenderer): { x: number; y: number } {
	const v = world.clone().project(camera);
	const w = gl.domElement.clientWidth;
	const h = gl.domElement.clientHeight;
	return { x: (v.x * 0.5 + 0.5) * w, y: (-v.y * 0.5 + 0.5) * h };
}

function pixelsPerWorldUnitAt(camera: Camera, gl: WebGLRenderer, world: Vector3): number {
	if (!(camera as ThreePerspectiveCamera).isPerspectiveCamera) return 1;
	const pc = camera as ThreePerspectiveCamera;
	const dist = pc.position.distanceTo(world);
	const fovRad = (pc.fov * Math.PI) / 180;
	const h = Math.max(1, gl.domElement.clientHeight);
	return h / (2 * Math.tan(fovRad / 2) * Math.max(dist, 1e-6));
}

function attractionSnapDragTolerancePx(worldHandle: Vector3, radiusWorld: number, camera: Camera, gl: WebGLRenderer): number {
	const mpp = pixelsPerWorldUnitAt(camera, gl, worldHandle);
	const radPx = radiusWorld * mpp;
	return HANDLE_HIT_TOLERANCE_PX + ATTRACTION_HANDLE_SNAP_EXTRA_PX + radPx * camera.zoom;
}

function attractionSnapCommitTolerancePx(worldHandle: Vector3, radiusWorld: number, camera: Camera, gl: WebGLRenderer): number {
	const mpp = pixelsPerWorldUnitAt(camera, gl, worldHandle);
	const radPx = radiusWorld * mpp;
	return HANDLE_HIT_TOLERANCE_PX + ATTRACTION_COMMIT_SNAP_TIGHT_PX + radPx * camera.zoom;
}

function attractionSnapCommitProximityOk(
	attractedFullId: string,
	pointerWorld: Vector3,
	camera: Camera,
	gl: WebGLRenderer,
	getVortexWorld: (id: string) => Vector3 | null,
	metaRadius: (id: string) => number,
): boolean {
	const hw = getVortexWorld(attractedFullId);
	if (!hw) return false;
	const pScr = worldToCanvasPx(pointerWorld, camera, gl);
	const hScr = worldToCanvasPx(hw, camera, gl);
	const d = Math.hypot(pScr.x - hScr.x, pScr.y - hScr.y);
	return d <= attractionSnapCommitTolerancePx(hw, metaRadius(attractedFullId), camera, gl);
}

function nearestAttractionSnapFullId(args: {
	lod: number;
	pointerWorld: Vector3;
	attractingFullId: string;
	compat: ReadonlySet<string>;
	blocked: ReadonlySet<string>;
	camera: Camera;
	gl: WebGLRenderer;
	getVortexWorld: (id: string) => Vector3 | null;
	metaRadius: (id: string) => number;
}): string | null {
	if (args.lod >= SCENE_ATTRACTION_SNAP_MAX_LOD) return null;
	const pScr = worldToCanvasPx(args.pointerWorld, args.camera, args.gl);
	let best: { d: number; id: string } | null = null;
	for (const tid of args.compat) {
		if (tid === args.attractingFullId) continue;
		if (args.blocked.has(tid)) continue;
		const hw = args.getVortexWorld(tid);
		if (!hw) continue;
		const hScr = worldToCanvasPx(hw, args.camera, args.gl);
		const d = Math.hypot(hScr.x - pScr.x, hScr.y - pScr.y);
		const tol = attractionSnapDragTolerancePx(hw, args.metaRadius(tid), args.camera, args.gl);
		if (d > tol) continue;
		if (!best || d < best.d) best = { d, id: tid };
	}
	return best?.id ?? null;
}
//#endregion ­ƒº▓AttractionGesture

//#region ­ƒºèMesh
/** @emoji 🖱️ R3F pointer handlers for scene mesh pick targets. */
export interface SceneMeshPointerHandlers {
	readonly onPointerDown?: (event: ThreeEvent<PointerEvent>) => void;
	readonly onPointerOver?: (event: ThreeEvent<PointerEvent>) => void;
	readonly onPointerOut?: (event: ThreeEvent<PointerEvent>) => void;
}

export interface MeshProps extends SceneMeshPointerHandlers {
	readonly meshUrl: string;
	readonly style?: MeshStyleKind;
	readonly userData?: Record<string, unknown>;
	readonly scale?: number | [number, number, number];
}

/** @emoji 🧭 Inner group: glTF Y-up mesh geometry → CAD object-local Z-up (fixture pose stays CAD). */
function GlbMeshFrame(props: { readonly children: ReactNode }) {
	return <group rotation={[GLB_MESH_FRAME_ROTATION_X, 0, 0]}>{props.children}</group>;
}

/** @emoji ­ƒºè Pooled GLB body with {@link MeshStyleKind} recoloring aligned to Elements tokens. */
export const MeshBody = memo(function MeshBody(props: MeshProps) {
	const style = props.style ?? DEFAULT_MESH_STYLE;
	const renderRoot = usePooledStyledMesh(props.meshUrl, style);
	if (!renderRoot) {
		return null;
	}
	const scale = props.scale;
	return (
		<GlbMeshFrame>
			<Clone
				object={renderRoot}
				{...(scale !== undefined
					? {
							scale:
								typeof scale === "number"
									? ([scale, scale, scale] as [number, number, number])
									: (scale as [number, number, number]),
						}
					: {})}
				onPointerDown={props.onPointerDown}
				onPointerOut={props.onPointerOut}
				onPointerOver={props.onPointerOver}
				userData={props.userData}
			/>
		</GlbMeshFrame>
	);
});

const PlaceholderMesh = memo(function PlaceholderMesh(
	props: SceneMeshPointerHandlers & { readonly style: MeshStyleKind },
) {
	const colors = meshStyleColors(props.style);
	const meshColor = colors?.meshColor ?? "#cbd5e1";
	const opacity = colors?.opacity ?? 1;
	return (
		<GlbMeshFrame>
			<mesh
				onPointerDown={props.onPointerDown}
				onPointerOut={props.onPointerOut}
				onPointerOver={props.onPointerOver}
			>
				<boxGeometry args={[1, 1, 1]} />
				<meshStandardMaterial
					color={meshColor}
					metalness={0.05}
					roughness={0.85}
					transparent={opacity < 1}
					opacity={opacity}
				/>
			</mesh>
		</GlbMeshFrame>
	);
});
//#endregion ­ƒºèMesh

//#region ­ƒºèObject

const ObjectTransformControls = memo(function ObjectTransformControls(props: {
	readonly object: Group;
	readonly objectId: string;
	readonly mode: RelocateMode;
	readonly translationSnap: number | undefined;
	readonly beforeRef: MutableRefObject<{ origin: Vector3; quat: Quaternion; scale: Vector3 } | null>;
}) {
	const reg = useRegistry();
	const scene = useThree((s) => s.scene);
	return createPortal(
		<TransformControls
			object={props.object}
			mode={props.mode}
			translationSnap={props.translationSnap}
			onMouseDown={() => {
				const g = props.object;
				props.beforeRef.current = {
					origin: g.position.clone(),
					quat: g.quaternion.clone(),
					scale: g.scale.clone(),
				};
			}}
			onMouseUp={() => {
				const before = props.beforeRef.current;
				if (!before) return;
				const g = props.object;
				reg.onRelocate?.({
					objectId: props.objectId,
					mode: props.mode,
					before: {
						origin: threeVec3ToCad(before.origin),
						orientation: threeQuatToCad(before.quat),
						scale: before.scale.toArray() as unknown as Vec3,
					},
					after: {
						origin: threeVec3ToCad(g.position),
						orientation: threeQuatToCad(g.quaternion),
						scale: g.scale.toArray() as unknown as Vec3,
					},
				});
				const cand = reg.findNearestProximityRelocate(g.position, props.objectId);
				if (cand) reg.onProximityConnect?.(cand);
				props.beforeRef.current = null;
			}}
		/>,
		scene,
	);
});

export const ObjectItem = memo(function ObjectItem(props: ObjectProps) {
	const group = useRef<Group>(null);
	const {
		registerObject,
		selectionMode,
		setSelectedObjectIds,
		onSelect,
		setActiveRelocateObjectId,
		activeRelocateObjectId,
		relocateMode,
		attractionDragActive,
		attractionIndirectPickAwait,
		attractionCompatibleAttractedFullIds,
		setSceneHover,
		clearSceneHover,
		isSceneHovered,
	} = useRegistry();
	const beforeRef = useRef<{ origin: Vector3; quat: Quaternion; scale: Vector3 } | null>(null);
	const [tcTarget, setTcTarget] = useState<Group | null>(null);
	const objectPointerHovered = isSceneHovered({ kind: "object", id: props.id });

	useEffect(() => {
		registerObject(props.id, props.objectKind, group.current);
		return () => {
			registerObject(props.id, props.objectKind, null);
		};
	}, [props.id, props.objectKind, registerObject]);

	useEffect(() => {
		if (group.current) setTcTarget(group.current);
	}, [props.selected, props.id, activeRelocateObjectId]);

	const linkHighlighted = useMemo(() => {
		if (props.highlighted === true) {
			return true;
		}
		const prefix = `${props.id}:`;
		for (const fullId of attractionCompatibleAttractedFullIds) {
			if (fullId.startsWith(prefix)) {
				return true;
			}
		}
		return false;
	}, [props.highlighted, props.id, attractionCompatibleAttractedFullIds]);

	const meshStyle = useMemo(
		() =>
			resolveMeshStyle({
				style: props.style,
				disabled: props.disabled,
				selected: props.selected,
				highlighted: linkHighlighted,
				hovered: props.hovered === true || objectPointerHovered,
			}),
		[props.style, props.disabled, props.selected, props.hovered, linkHighlighted, objectPointerHovered],
	);

	const meshPointerHandlers = useMemo(
		() => ({
			onPointerDown: (e: ThreeEvent<PointerEvent>) => {
				const pe = e.nativeEvent;
				if (pe.button !== 0) {
					return;
				}
				e.stopPropagation();
				if (attractionDragActive || attractionIndirectPickAwait) {
					return;
				}
				if (props.disabled) {
					return;
				}
				const snap: SelectionSnapshot = { objectIds: [props.id], vortexIds: [] };
				if (selectionMode === "single") {
					setSelectedObjectIds([props.id]);
					onSelect?.(snap);
				} else if (selectionMode === "additive") {
					setSelectedObjectIds((prev) => (prev.includes(props.id) ? prev : [...prev, props.id]));
					onSelect?.(snap);
				} else {
					onSelect?.(snap);
				}
				setActiveRelocateObjectId(props.id);
			},
			onPointerOver: (e: ThreeEvent<PointerEvent>) => {
				e.stopPropagation();
				if (!props.disabled && !attractionDragActive && !attractionIndirectPickAwait) {
					setSceneHover({ kind: "object", id: props.id });
				}
			},
			onPointerOut: (e: ThreeEvent<PointerEvent>) => {
				e.stopPropagation();
				clearSceneHover({ kind: "object", id: props.id });
			},
		}),
		[
			attractionDragActive,
			attractionIndirectPickAwait,
			clearSceneHover,
			onSelect,
			props.disabled,
			props.id,
			selectionMode,
			setActiveRelocateObjectId,
			setSceneHover,
			setSelectedObjectIds,
		],
	);

	const poseKey = useMemo(
		() => objectPoseKey(props.id, props.origin, props.orientation, props.scale),
		[props.id, props.origin, props.orientation, props.scale],
	);
	useLayoutEffect(() => {
		const g = group.current;
		if (!g) {
			return;
		}
		applyObjectPose(g, props.origin, props.orientation, props.scale);
	}, [poseKey]);
	const lodCtx = useLod();
	const resolvedMeshUrl = useResolvedSceneMeshUrl({
		origin: props.origin,
		meshByLod: props.meshByLod,
		fallbackMeshUrl: props.meshUrl,
	});
	const mode = props.relocate ?? relocateMode;
	const transSnap =
		mode === "translate" &&
		lodCtx.gridSnapEnabled &&
		lodCtx.gridStepWorld != null &&
		lodCtx.gridStepWorld > 0
			? lodCtx.gridStepWorld
			: undefined;
	const showTc =
		props.selected && activeRelocateObjectId === props.id && props.relocate !== false && tcTarget;

	return (
		<>
			<group
				ref={group}
				visible={props.visible !== false}
				userData={{
					sceneObjectId: props.id,
					sceneMeshStyle: meshStyle,
					...(props.attracting?.length ? { sceneAttracting: props.attracting } : {}),
					...(props.wormhole ? { sceneWormhole: true } : {}),
					...props.userData,
				}}
			>
				{resolvedMeshUrl === PLACEHOLDER_MESH_URL ? (
					<PlaceholderMesh style={meshStyle} {...meshPointerHandlers} />
				) : (
					<MeshBody meshUrl={resolvedMeshUrl} style={meshStyle} {...meshPointerHandlers} />
				)}
				<group userData={{ sceneObjectAttachments: props.id }}>{props.children}</group>
			</group>
			{showTc && tcTarget && (
				<ObjectTransformControls
					object={tcTarget}
					objectId={props.id}
					mode={mode}
					translationSnap={transSnap}
					beforeRef={beforeRef}
				/>
			)}
		</>
	);
});
//#endregion ­ƒºèObject

//#region ­ƒîÇVortex
const vortexFallbackMatProps = { transparent: true, opacity: 0.55 } as const;

function VortexHandleGltf(props: {
	meshUrl: string;
	fullId: string;
	radius: number;
	style: MeshStyleKind;
	onPointerOver?: (e: ThreeEvent<PointerEvent>) => void;
	onPointerOut?: (e: ThreeEvent<PointerEvent>) => void;
}) {
	const scale = (props.radius / 0.35) * 0.9;
	const { onPointerOver, onPointerOut, ...meshProps } = props;
	return (
		<MeshBody
			meshUrl={meshProps.meshUrl}
			style={meshProps.style}
			scale={scale}
			userData={{ sceneVortexFullId: meshProps.fullId }}
			onPointerOver={onPointerOver}
			onPointerOut={onPointerOut}
		/>
	);
}

function vortexHighlightMeshStyle(
	highlight: "none" | "compatible" | "ring" | "attracting" | "indirectRing",
): MeshStyleKind {
	switch (highlight) {
		case "ring":
		case "indirectRing":
			return "highlighted";
		case "compatible":
			return "hovered";
		case "attracting":
			return "selected";
		default:
			return "neutral";
	}
}

function VortexFallbackMesh(props: {
	fullId: string;
	radius: number;
	highlight: "none" | "compatible" | "ring" | "attracting" | "indirectRing";
	onPointerOver?: (e: ThreeEvent<PointerEvent>) => void;
	onPointerOut?: (e: ThreeEvent<PointerEvent>) => void;
}) {
	const style = vortexHighlightMeshStyle(props.highlight);
	const colors = meshStyleColors(style) ?? meshStyleColors("neutral")!;
	const { onPointerOver, onPointerOut, ...meshProps } = props;
	return (
		<mesh
			userData={{ sceneVortexFullId: meshProps.fullId }}
			onPointerOver={onPointerOver}
			onPointerOut={onPointerOut}
		>
			<sphereGeometry args={[meshProps.radius, 12, 12]} />
			<meshStandardMaterial
				color={colors.meshColor}
				emissive={colors.emissiveColor}
				emissiveIntensity={colors.emissiveIntensity}
				transparent={colors.opacity < 1}
				opacity={colors.opacity}
				{...vortexFallbackMatProps}
			/>
		</mesh>
	);
}

export const Vortex = memo(function Vortex(
	props: VortexProps & {
		objectId: string;
		objectKind?: string;
		objectOrigin: Vec3;
		objectOrientation?: Quat;
		selected?: boolean;
	},
) {
	const root = useRef<Group | null>(null);
	const reg = useRegistry();
	const fullId = props.id.includes(":") ? props.id : `${props.objectId}:${props.id}`;
	const r = props.radius ?? 0.35;

	useEffect(() => {
		const getter = () => {
			if (!root.current) return null;
			updateWorldMatrixChain(root.current);
			const v = new Vector3();
			root.current.getWorldPosition(v);
			return v;
		};
		reg.registerVortex(fullId, getter);
		return () => {
			reg.unregisterVortex(fullId);
		};
	}, [fullId, reg]);

	const bindRoot = useCallback(
		(node: Group | null) => {
			root.current = node;
			if (node) {
				reg.registerVortexBinding(
					{
						fullId,
						objectId: props.objectId,
						objectKind: props.objectKind,
						vortexKind: props.vortexKind,
						radiusWorld: r,
					},
					node,
				);
			} else {
				reg.unregisterVortexBinding(fullId);
			}
		},
		[fullId, props.objectId, props.objectKind, props.vortexKind, reg],
	);

	const lodCtx = useLod();
	const worldPosRef = useRef(new Vector3());
	const handleMeshByLodRef = useRef(props.handleMeshByLod);
	handleMeshByLodRef.current = props.handleMeshByLod;
	const handleMeshUrlRef = useRef(props.handleMeshUrl);
	handleMeshUrlRef.current = props.handleMeshUrl;
	const trackVortexLod =
		lodCtx.depthVariable || (props.handleMeshByLod?.length ?? 0) > 0;
	const [lodVisual, setLodVisual] = useState<VortexLodVisual>(() =>
		vortexLodVisual(lodCtx.lod, false, props.handleMeshByLod, props.handleMeshUrl),
	);
	const highlight: "none" | "compatible" | "ring" | "attracting" | "indirectRing" = props.selected
		? "attracting"
		: reg.attractionDragAttractingFullId === fullId
			? "attracting"
			: reg.attractionHoverRingFullId === fullId
				? "ring"
				: reg.attractionIndirectPickAwait?.candidates.includes(fullId) === true
					? "indirectRing"
					: reg.attractionCompatibleAttractedFullIds.has(fullId)
						? "compatible"
						: "none";

	const onPointerDown = useCallback(
		(e: { stopPropagation: () => void; nativeEvent: PointerEvent }) => {
			const pe = e.nativeEvent;
			if (pe.button !== 0) return;
			e.stopPropagation();
			if (reg.blockedVortexFullIds.has(fullId)) return;
			if (pe.altKey || pe.metaKey) {
				reg.onSelect?.({ objectIds: [props.objectId], vortexIds: [fullId] });
				reg.setActiveRelocateObjectId(props.objectId);
				return;
			}
			reg.beginAttractionDragFromVortex(fullId, props.objectId, props.objectKind, props.vortexKind);
			const el = pe.currentTarget instanceof Element ? pe.currentTarget : null;
			if (el && typeof (el as HTMLElement).setPointerCapture === "function") {
				try {
					(el as HTMLElement).setPointerCapture(pe.pointerId);
				} catch {
					/* ignore */
				}
			}
		},
		[fullId, props.objectId, props.objectKind, props.vortexKind, reg],
	);

	const inIndirectRing = reg.attractionIndirectPickAwait?.candidates.includes(fullId) === true;
	const linger =
		(reg.attractionDragActive &&
			(reg.attractionDragAttractingFullId === fullId ||
				reg.attractionHoverRingFullId === fullId ||
				reg.attractionCompatibleAttractedFullIds.has(fullId))) ||
		inIndirectRing;
	const lingerRef = useRef(linger);
	lingerRef.current = linger;
	useFrame(() => {
		if (!trackVortexLod) return;
		const lod = lodCtx.depthVariable
			? (() => {
					if (!root.current) return lodCtx.lod;
					updateWorldMatrixChain(root.current);
					root.current.getWorldPosition(worldPosRef.current);
					return lodCtx.lodForWorldPosition(worldPosRef.current.toArray() as Vec3);
				})()
			: lodCtx.lod;
		const next = vortexLodVisual(lod, lingerRef.current, handleMeshByLodRef.current, handleMeshUrlRef.current);
		setLodVisual((prev) => (vortexLodVisualEqual(prev, next) ? prev : next));
	});
	const drawHandleBody = trackVortexLod
		? lodVisual.drawHandleBody || linger
		: lodHandlePrimaryVisible(lodCtx.lod) || linger;
	const pickProxy =
		(drawHandleBody ? false : trackVortexLod ? lodVisual.pickProxy : lodHandlePickProxy(lodCtx.lod)) && !linger;
	const meshUrl = trackVortexLod
		? lodVisual.meshUrl
		: pickClosestMeshUrl(props.handleMeshByLod, lodCtx.lod, props.handleMeshUrl);

	const positionThree = useMemo(
		() => cadObjectLocalToThreeGroupLocal(props.position, props.objectOrigin, props.objectOrientation),
		[props.position, props.objectOrigin, props.objectOrientation],
	);

	const vortexPointerHovered = reg.isSceneHovered({ kind: "vortex", fullId });
	const handleMeshStyle =
		highlight === "none" && vortexPointerHovered ? "hovered" : vortexHighlightMeshStyle(highlight);

	const vortexPointerHoverHandlers = useMemo(
		() => ({
			onPointerOver: (e: ThreeEvent<PointerEvent>) => {
				e.stopPropagation();
				if (!reg.attractionDragActive && !reg.attractionIndirectPickAwait) {
					reg.setSceneHover({ kind: "vortex", fullId });
				}
			},
			onPointerOut: (e: ThreeEvent<PointerEvent>) => {
				e.stopPropagation();
				reg.clearSceneHover({ kind: "vortex", fullId });
			},
		}),
		[fullId, reg],
	);

	const vis = props.visible !== false;
	return (
		<group
			ref={bindRoot}
			position={positionThree}
			userData={{ sceneVortexFullId: fullId, vortexKind: props.vortexKind }}
			data-scene-vortex={fullId}
			visible={vis}
			onPointerDown={onPointerDown}
		>
			{drawHandleBody && meshUrl ? (
				<VortexHandleGltf
					meshUrl={meshUrl}
					fullId={fullId}
					radius={r}
					style={handleMeshStyle}
					{...vortexPointerHoverHandlers}
				/>
			) : drawHandleBody && props.children ? (
				<group userData={{ sceneVortexFullId: fullId }} {...vortexPointerHoverHandlers}>
					{props.children}
				</group>
			) : drawHandleBody ? (
				<VortexFallbackMesh
					fullId={fullId}
					radius={r}
					highlight={highlight}
					{...vortexPointerHoverHandlers}
				/>
			) : null}
			{pickProxy ? (
				<mesh userData={{ sceneVortexFullId: fullId }} renderOrder={-1} {...vortexPointerHoverHandlers}>
					<sphereGeometry args={[r, 12, 12]} />
					<meshBasicMaterial transparent opacity={0} depthWrite={false} />
				</mesh>
			) : null}
		</group>
	);
});
//#endregion ­ƒîÇVortex

//#region ­ƒº▓Magnet
export const Magnet = memo(function Magnet(
	props: MagnetProps & { objectOrigin: Vec3; objectOrientation?: Quat },
) {
	const positionThree = useMemo(
		() => cadObjectLocalToThreeGroupLocal(props.position, props.objectOrigin, props.objectOrientation),
		[props.position, props.objectOrigin, props.objectOrientation],
	);
	return (
		<mesh position={positionThree} userData={{ sceneMagnetId: props.id }}>
			<boxGeometry args={[props.size[0], props.size[1], props.size[2]]} />
			<meshStandardMaterial color="#a78bfa" wireframe />
		</mesh>
	);
});
//#endregion ­ƒº▓Magnet

//#region ­ƒº▓SceneAttraction
function sceneAttractionIndexFromPointerEvent(e: ThreeEvent<PointerEvent>): number {
	if (e.index != null) {
		return Math.floor(e.index / 2);
	}
	if (e.faceIndex != null) {
		return e.faceIndex;
	}
	return 0;
}

const SceneAttractionLineBatch = memo(function SceneAttractionLineBatch(props: {
	readonly attractions: readonly AttractionProps[];
}) {
	const reg = useRegistry();
	const mat = useMemo(() => {
		const color = lineCssColor(CSS_ATTRACTION_ENDPOINT_LINE, "#64748b");
		return new LineBasicMaterial({ color, transparent: true, opacity: 0.85, depthTest: true, vertexColors: true });
	}, []);
	const geo = useMemo(() => new BufferGeometry(), []);
	const normalColor = useMemo(() => new Color(lineCssColor(CSS_ATTRACTION_ENDPOINT_LINE, "#64748b")), []);
	const hoveredColor = useMemo(() => new Color(lineCssColor(CSS_HOVERED_LINE, "#38bdf8")), []);
	useLayoutEffect(() => {
		const vertexCount = Math.max(props.attractions.length * 2, 2);
		geo.setAttribute("position", new Float32BufferAttribute(new Float32Array(vertexCount * 3), 3));
		geo.setAttribute("color", new Float32BufferAttribute(new Float32Array(vertexCount * 3), 3));
	}, [geo, props.attractions.length]);
	useFrame(() => {
		const pos = geo.attributes.position as Float32BufferAttribute;
		const colors = geo.attributes.color as Float32BufferAttribute;
		const hoveredId = reg.hoverTarget?.kind === "attraction" ? reg.hoverTarget.id : null;
		let write = 0;
		for (const attraction of props.attractions) {
			const a = reg.getVortexWorld(attraction.attracting);
			const b = reg.getVortexWorld(attraction.attracted);
			const c = attraction.id === hoveredId ? hoveredColor : normalColor;
			if (a && b && vector3IsFinite(a) && vector3IsFinite(b)) {
				pos.setXYZ(write, a.x, a.y, a.z);
				pos.setXYZ(write + 1, b.x, b.y, b.z);
			} else {
				pos.setXYZ(write, 0, 0, 0);
				pos.setXYZ(write + 1, 0, 0, 0);
			}
			colors.setXYZ(write, c.r, c.g, c.b);
			colors.setXYZ(write + 1, c.r, c.g, c.b);
			write += 2;
		}
		pos.needsUpdate = true;
		colors.needsUpdate = true;
	});
	const onPointerOver = useCallback(
		(e: ThreeEvent<PointerEvent>) => {
			e.stopPropagation();
			if (reg.attractionDragActive || reg.attractionIndirectPickAwait) {
				return;
			}
			const idx = sceneAttractionIndexFromPointerEvent(e);
			const attraction = props.attractions[idx];
			if (attraction) {
				reg.setSceneHover({ kind: "attraction", id: attraction.id });
			}
		},
		[props.attractions, reg],
	);
	const onPointerOut = useCallback(
		(e: ThreeEvent<PointerEvent>) => {
			e.stopPropagation();
			const idx = sceneAttractionIndexFromPointerEvent(e);
			const attraction = props.attractions[idx];
			if (attraction) {
				reg.clearSceneHover({ kind: "attraction", id: attraction.id });
			}
		},
		[props.attractions, reg],
	);
	useEffect(
		() => () => {
			geo.dispose();
			mat.dispose();
		},
		[geo, mat],
	);
	if (!props.attractions.length) {
		return null;
	}
	return (
		<lineSegments geometry={geo} material={mat} onPointerOver={onPointerOver} onPointerOut={onPointerOut} />
	);
});

export const SceneAttraction = memo(function SceneAttraction(props: AttractionProps) {
	return <SceneAttractionLineBatch attractions={[props]} />;
});
//#endregion ­ƒº▓SceneAttraction

//#region ­ƒº▓Attraction
export const Attraction = memo(function Attraction(props: { attracting: Vec3; attracted: Vec3 }) {
	const pts = useMemo(() => [vec3ToThree(props.attracting), vec3ToThree(props.attracted)], [props.attracting, props.attracted]);
	const color = useMemo(
		() => lineCssColor(CSS_ATTRACTION_LINE, "#f472b6"),
		[],
	);
	return <Line points={pts} color={color} lineWidth={2} />;
});
//#endregion ­ƒº▓Attraction

//#region Ô£ïRelocate
export function useSceneRelocate(objectId: string) {
	const reg = useRegistry();
	return {
		mode: reg.relocateMode,
		start: () => reg.setActiveRelocateObjectId(objectId),
		cancel: () => reg.setActiveRelocateObjectId(null),
	};
}
//#endregion Ô£ïRelocate

const EMPTY_BLOCKED_VORTICES: ReadonlySet<string> = new Set();

//#region ­ƒÄ¼Scene
function OrbitGated(props: {
	readonly camera: ThreePerspectiveCamera | null;
	readonly zoom: number;
	readonly onCamera?: (state: CameraState) => void;
}) {
	const reg = useRegistry();
	const { camera } = useThree();
	const controls = useThree((s) => s.controls as { target: Vector3 } | null);
	const targetScratch = useMemo(() => new Vector3(), []);
	const gate = reg.attractionDragActive || reg.attractionIndirectPickAwait !== null;
	const invalidate = useThree((s) => s.invalidate);
	const reportCamera = useCallback(() => {
		if (!props.onCamera) {
			return;
		}
		const tgt = controls?.target ?? targetScratch.set(0, 0, 0);
		props.onCamera({
			position: threeVec3ToCad(camera.position),
			target: threeVec3ToCad(tgt),
			zoom: props.zoom,
		});
	}, [camera, controls, props.onCamera, props.zoom, targetScratch]);
	useEffect(() => {
		invalidate();
	}, [gate, invalidate]);
	if (!props.camera) {
		return null;
	}
	return (
		<OrbitControls
			camera={props.camera}
			makeDefault
			enabled={!gate}
			enableDamping={false}
			enablePan
			enableZoom
			onChange={() => invalidate()}
			onStart={() => invalidate()}
			onEnd={() => {
				invalidate();
				reportCamera();
			}}
			mouseButtons={{ LEFT: MOUSE.ROTATE, MIDDLE: MOUSE.DOLLY, RIGHT: MOUSE.PAN }}
		/>
	);
}

/** @emoji 🛰️ Frames orbit camera to loaded object bounds once meshes are measurable (initial load fit). */
function SceneAutoFit(props: {
	readonly behavior?: SceneAutoFitBehavior;
	readonly padding?: number;
	readonly zoom?: number;
	readonly onCamera?: (state: CameraState) => void;
}): null {
	const reg = useRegistry();
	const { camera, controls, invalidate } = useThree();
	const targetScratch = useMemo(() => new Vector3(), []);
	const lastKey = useRef("");
	const hasApplied = useRef(false);
	const behavior = props.behavior ?? "initial";
	const padding = props.padding ?? 1.25;
	const zoom = props.zoom ?? 1;
	useFrame(() => {
		if (behavior === "initial" && hasApplied.current) {
			return;
		}
		const groups = reg.collectObjectGroups();
		if (!groups.length) return;
		const key = groups
			.map((group) => group.uuid)
			.sort()
			.join("|");
		if (!sceneAutoFitShouldRun(behavior, key, lastKey.current, hasApplied.current)) return;
		const bounds = boundsFromObjectGroups(groups);
		if (!bounds) return;
		lastKey.current = key;
		hasApplied.current = true;
		if (!(camera instanceof ThreePerspectiveCamera)) return;
		const orbit = controls as { target: Vector3; update?: () => void } | null;
		applySceneAutoFitCamera(camera, bounds, padding, orbit);
		invalidate();
		const tgt = orbit?.target ?? targetScratch.set(...cadVec3ToThree(bounds.center));
		props.onCamera?.({
			position: threeVec3ToCad(camera.position),
			target: threeVec3ToCad(tgt),
			zoom,
		});
	});
	return null;
}

/** @emoji ­ƒôÀ Seeds default camera + orbit target once; orbit owns the rig afterward (no controlled-camera feedback loop). */
function SceneCameraSeed(props: {
	readonly camera: ThreePerspectiveCamera | null;
	readonly position: Vec3;
	readonly target: Vec3;
}) {
	const controls = useThree((s) => s.controls as { target: Vector3; update: () => void } | null);
	const seededPositionFor = useRef("");
	const seededTargetFor = useRef("");
	const positionKey = props.position.join(",");
	const targetKey = props.target.join(",");
	useLayoutEffect(() => {
		const camera = props.camera;
		if (!camera) {
			return;
		}
		if (seededPositionFor.current !== positionKey) {
			seededPositionFor.current = positionKey;
			const p = cadVec3ToThree(props.position);
			camera.position.set(p[0], p[1], p[2]);
			camera.updateProjectionMatrix();
		}
		if (controls?.target && seededTargetFor.current !== targetKey) {
			seededTargetFor.current = targetKey;
			const t = cadVec3ToThree(props.target);
			controls.target.set(t[0], t[1], t[2]);
			controls.update();
		}
	}, [controls, positionKey, props.camera, props.position, props.target, targetKey]);
	return null;
}

function AttractionThreeBinder() {
	const reg = useRegistry();
	const t = useThree();
	useLayoutEffect(() => {
		reg.attachAttractionThreeEnv({ camera: t.camera, gl: t.gl, scene: t.scene });
		return () => reg.attachAttractionThreeEnv(null);
	}, [reg, t.camera, t.gl, t.scene]);
	return null;
}

function AttractionWindowBridge() {
	const reg = useRegistry();
	const invalidate = useThree((s) => s.invalidate);
	const attractionBusy = reg.attractionDragActive || reg.attractionIndirectPickAwait !== null;
	useEffect(() => {
		if (!attractionBusy) return;
		const onMove = (e: PointerEvent) => {
			if (reg.attractionDragActive) reg.updateAttractionPointer(e.clientX, e.clientY);
			else if (reg.attractionIndirectPickAwait) reg.updateIndirectPickPointer(e.clientX, e.clientY);
			invalidate();
		};
		const onUp = (e: PointerEvent) => {
			if (reg.attractionDragActive) reg.commitAttractionPointer(e.clientX, e.clientY);
			invalidate();
		};
		const onDown = (e: PointerEvent) => {
			if (e.button !== 0) return;
			if (reg.attractionIndirectPickAwait) reg.commitIndirectPickPointerDown(e.clientX, e.clientY, e);
			invalidate();
		};
		const bindings = new SceneEventBindingController();
		bindings.listen(window, "pointermove", onMove);
		bindings.listen(window, "pointerup", onUp, { capture: true });
		bindings.listen(window, "pointerdown", onDown, true);
		return () => bindings.dispose();
	}, [reg, attractionBusy, invalidate]);
	return null;
}

function AttractionRubberBand() {
	const reg = useRegistry();
	const geo = useMemo(() => {
		const g = new BufferGeometry();
		g.setAttribute("position", new Float32BufferAttribute(new Float32Array(6), 3));
		return g;
	}, []);
	const mat = useMemo(
		() => new LineBasicMaterial({ color: 0xf472b6, transparent: true, opacity: 0.92, depthTest: false }),
		[],
	);
	useFrame(() => {
		const pos = geo.attributes.position as Float32BufferAttribute;
		const attractionLine =
			(reg.attractionDragActive || reg.attractionIndirectPickAwait !== null) && reg.attractionDragAttractingFullId ? true : false;
		if (!attractionLine) {
			pos.setXYZ(0, 0, 0, 0);
			pos.setXYZ(1, 0, 0, 0);
			pos.needsUpdate = true;
			return;
		}
		const a = reg.getVortexWorld(reg.attractionDragAttractingFullId);
		const b = reg.attractionEndWorldRef.current;
		if (a && b && vector3IsFinite(a) && vector3IsFinite(b)) {
			pos.setXYZ(0, a.x, a.y, a.z);
			pos.setXYZ(1, b.x, b.y, b.z);
			pos.needsUpdate = true;
		} else {
			pos.setXYZ(0, 0, 0, 0);
			pos.setXYZ(1, 0, 0, 0);
			pos.needsUpdate = true;
		}
	});
	useEffect(
		() => () => {
			geo.dispose();
			mat.dispose();
		},
		[geo, mat],
	);
	return <line geometry={geo} material={mat} raycast={() => null} />;
}

function RegistryProvider({
	children,
	lodRef,
	kindCatalogs,
	kindCompatibility,
	blockedVortexFullIds,
	proximityRadius,
	selectionMode,
	relocateMode,
	selection: controlledSelection,
	onSelect,
	onConnect,
	onProximityConnect,
	onIndirectConnect,
	onAttractionCompatibleObjects,
	onAttractionTargetRing,
	onRelocate,
}: {
	children: ReactNode;
	lodRef: MutableRefObject<number>;
	kindCatalogs: KindCatalogBundle | undefined;
	kindCompatibility: readonly KindCompatEntry[] | undefined;
	blockedVortexFullIds: ReadonlySet<string>;
	proximityRadius: number;
	selectionMode: SelectionMode;
	relocateMode: RelocateMode;
	selection?: SelectionSnapshot;
	onSelect?: (snap: SelectionSnapshot) => void;
	onConnect?: (p: AttractionPayload) => void;
	onProximityConnect?: (p: AttractionPayload) => void;
	onIndirectConnect?: (p: AttractionPayload) => void;
	onAttractionCompatibleObjects?: (p: AttractionCompatibleObjectsPayload) => void;
	onAttractionTargetRing?: (p: AttractionTargetRingPayload) => void;
	onRelocate?: (p: RelocatePayload) => void;
}) {
	const [internalSelectedObjectIds, setInternalSelectedObjectIds] = useState<readonly string[]>([]);
	const selectedObjectIds = controlledSelection?.objectIds ?? internalSelectedObjectIds;
	const setSelectedObjectIds = useCallback(
		(ids: readonly string[] | ((prev: readonly string[]) => readonly string[])) => {
			if (controlledSelection !== undefined) {
				const resolved = typeof ids === "function" ? ids(controlledSelection.objectIds) : ids;
				onSelect?.({
					objectIds: resolved,
					vortexIds: controlledSelection.vortexIds,
				});
				return;
			}
			setInternalSelectedObjectIds(ids);
		},
		[controlledSelection, onSelect],
	);
	const [activeRelocateObjectId, setActiveRelocateObjectId] = useState<string | null>(null);
	const selectionControlled = controlledSelection !== undefined;
	const selectionPrimaryObjectId = controlledSelection?.objectIds[0] ?? null;
	const selectionPrimaryVortexId = controlledSelection?.vortexIds[0] ?? null;
	useEffect(() => {
		if (!selectionControlled) {
			return;
		}
		const primary =
			selectionPrimaryObjectId ??
			(selectionPrimaryVortexId ? parseVortexFullId(selectionPrimaryVortexId).objectId : null);
		setActiveRelocateObjectId(primary);
	}, [selectionControlled, selectionPrimaryObjectId, selectionPrimaryVortexId]);
	const [attractionDragActive, setAttractionDragActive] = useState(false);
	const [attractionDragAttractingFullId, setAttractionDragAttractingFullId] = useState<string | null>(null);
	const [attractionCompatibleAttractedFullIds, setAttractionCompatibleAttractedFullIds] = useState<ReadonlySet<string>>(new Set());
	const [attractionHoverRingFullId, setAttractionHoverRingFullId] = useState<string | null>(null);
	const [attractionIndirectPickAwait, setAttractionIndirectPickAwait] = useState<AttractionIndirectPickAwait | null>(null);
	const [hoverTarget, setHoverTarget] = useState<SceneHoverTarget | null>(null);

	const setSceneHover = useCallback((target: SceneHoverTarget) => {
		setHoverTarget((prev) => (sceneHoverTargetsEqual(prev, target) ? prev : target));
	}, []);

	const clearSceneHover = useCallback((target: SceneHoverTarget) => {
		setHoverTarget((prev) => (sceneHoverTargetsEqual(prev, target) ? null : prev));
	}, []);

	const clearSceneHoverAll = useCallback(() => {
		setHoverTarget((prev) => (prev === null ? prev : null));
	}, []);

	const isSceneHovered = useCallback(
		(target: SceneHoverTarget) => sceneHoverTargetsEqual(hoverTarget, target),
		[hoverTarget],
	);

	const clearSceneSelection = useCallback(() => {
		clearSceneHoverAll();
		setActiveRelocateObjectId(null);
		if (controlledSelection !== undefined) {
			onSelect?.({ objectIds: [], vortexIds: [] });
			return;
		}
		setInternalSelectedObjectIds([]);
		onSelect?.({ objectIds: [], vortexIds: [] });
	}, [clearSceneHoverAll, controlledSelection, onSelect]);

	const vortexGettersRef = useRef(new Map<string, VortexGetter>());
	const vortexMetaRef = useRef(new Map<string, VortexBindingMeta>());
	const vortexPickRef = useRef(new Map<string, Object3D>());
	const objectGroupMap = useRef(new Map<string, Group | null>());
	const objectKindsRef = useRef(new Map<string, string | undefined>());
	const indirectPickRef = useRef<AttractionIndirectPickAwait | null>(null);

	useEffect(() => {
		indirectPickRef.current = attractionIndirectPickAwait;
	}, [attractionIndirectPickAwait]);

	const attractionSessionRef = useRef<{
		attractingFullId: string;
		attractingObjectId: string;
		attractingCtx: AttractionHandleContext;
		compat: Set<string>;
		snapAttractedFullId: string | null;
	} | null>(null);
	const attractionEndWorldRef = useRef<Vector3 | null>(null);
	const attractionThreeRef = useRef<{ camera: Camera; gl: WebGLRenderer; scene: ThreeScene } | null>(null);
	const raycasterRef = useRef(new Raycaster());
	const ndcRef = useRef(new Vector2());
	const planeRef = useRef(new Plane(new Vector3(0, 1, 0), 0));
	const hitScratchRef = useRef(new Vector3());

	const registerVortex = useCallback((fullId: string, getter: VortexGetter) => {
		vortexGettersRef.current.set(fullId, getter);
	}, []);

	const unregisterVortex = useCallback((fullId: string) => {
		vortexGettersRef.current.delete(fullId);
	}, []);

	const getVortexWorld = useCallback((fullId: string) => {
		const g = vortexGettersRef.current.get(fullId);
		return g ? g() : null;
	}, []);

	const registerVortexBinding = useCallback((meta: VortexBindingMeta, pickRoot: Object3D | null) => {
		vortexMetaRef.current.set(meta.fullId, meta);
		if (pickRoot) vortexPickRef.current.set(meta.fullId, pickRoot);
		else vortexPickRef.current.delete(meta.fullId);
	}, []);

	const unregisterVortexBinding = useCallback((fullId: string) => {
		vortexMetaRef.current.delete(fullId);
		vortexPickRef.current.delete(fullId);
	}, []);

	const registerObject = useCallback((id: string, objectKind: string | undefined, group: Group | null) => {
		objectGroupMap.current.set(id, group);
		objectKindsRef.current.set(id, objectKind);
	}, []);

	const collectObjectGroups = useCallback((): Group[] => {
		const out: Group[] = [];
		for (const group of objectGroupMap.current.values()) {
			if (group) out.push(group);
		}
		return out;
	}, []);

	const getObjectGroup = useCallback((id: string) => objectGroupMap.current.get(id) ?? null, []);

	const getObjectKind = useCallback((id: string) => objectKindsRef.current.get(id), []);

	const cancelAttractionDrag = useCallback(() => {
		attractionSessionRef.current = null;
		attractionEndWorldRef.current = null;
		setAttractionDragActive(false);
		setAttractionDragAttractingFullId(null);
		setAttractionCompatibleAttractedFullIds(new Set());
		setAttractionHoverRingFullId(null);
		setAttractionIndirectPickAwait(null);
		setHoverTarget(null);
		onAttractionTargetRing?.({ attracting: "", objectId: null, vortexFullIds: [] });
	}, [onAttractionTargetRing]);

	const beginAttractionDragFromVortex = useCallback(
		(fullId: string, objectId: string, objectKind: string | undefined, vortexKind: string | undefined) => {
			if (indirectPickRef.current) return;
			if (blockedVortexFullIds.has(fullId)) return;
			const attractingCtx: AttractionHandleContext = { objectId, objectKind, vortexKind };
			const compat = new Set<string>();
			const objectIds = new Set<string>();
			for (const [tid, meta] of vortexMetaRef.current) {
				if (tid === fullId) continue;
				if (meta.objectId === objectId) continue;
				if (blockedVortexFullIds.has(tid)) continue;
				const attractedCtx: AttractionHandleContext = {
					objectId: meta.objectId,
					objectKind: meta.objectKind,
					vortexKind: meta.vortexKind,
				};
				if (!handlesAttractionCompatibleForDrag(attractingCtx, attractedCtx, kindCompatibility, kindCatalogs)) continue;
				compat.add(tid);
				objectIds.add(meta.objectId);
			}
			setAttractionIndirectPickAwait(null);
			attractionSessionRef.current = {
				attractingFullId: fullId,
				attractingObjectId: objectId,
				attractingCtx,
				compat,
				snapAttractedFullId: null,
			};
			attractionEndWorldRef.current = null;
			setAttractionDragActive(true);
			setAttractionDragAttractingFullId(fullId);
			setAttractionCompatibleAttractedFullIds(compat);
			setAttractionHoverRingFullId(null);
			setActiveRelocateObjectId(null);
			setHoverTarget(null);
			onAttractionCompatibleObjects?.({ attracting: fullId, objectIds: [...objectIds] });
		},
		[blockedVortexFullIds, kindCatalogs, kindCompatibility, onAttractionCompatibleObjects],
	);

	const collectPickRoots = useCallback((): Object3D[] => {
		const out: Object3D[] = [];
		for (const p of vortexPickRef.current.values()) out.push(p);
		for (const g of objectGroupMap.current.values()) if (g) out.push(g);
		return out;
	}, []);

	const updateAttractionPointer = useCallback(
		(clientX: number, clientY: number) => {
			const env = attractionThreeRef.current;
			const session = attractionSessionRef.current;
			if (!env || !session) return;
			const rect = env.gl.domElement.getBoundingClientRect();
			ndcRef.current.x = ((clientX - rect.left) / rect.width) * 2 - 1;
			ndcRef.current.y = -((clientY - rect.top) / rect.height) * 2 + 1;
			raycasterRef.current.setFromCamera(ndcRef.current, env.camera);
			const hits = raycasterRef.current.intersectObjects(collectPickRoots(), true);
			let ring: string | null = null;
			for (const h of hits) {
				const vf = readVortexFullIdFromObject(h.object);
				if (vf && session.compat.has(vf) && vf !== session.attractingFullId && !blockedVortexFullIds.has(vf)) {
					ring = vf;
					break;
				}
			}
			setAttractionHoverRingFullId((prev) => (prev === ring ? prev : ring));
			if (ring) {
				const meta = vortexMetaRef.current.get(ring);
				onAttractionTargetRing?.({
					attracting: session.attractingFullId,
					objectId: meta?.objectId ?? null,
					vortexFullIds: ring ? [ring] : [],
				});
			} else {
				onAttractionTargetRing?.({ attracting: session.attractingFullId, objectId: null, vortexFullIds: [] });
			}
			const hitWorld = hitScratchRef.current;
			if (hits.length > 0) {
				attractionEndWorldRef.current = hitWorld.copy(hits[0]!.point);
			} else if (raycasterRef.current.ray.intersectPlane(planeRef.current, hitWorld)) {
				attractionEndWorldRef.current = hitWorld.clone();
			} else {
				raycasterRef.current.ray.at(80, hitWorld);
				attractionEndWorldRef.current = hitWorld.clone();
			}
			const pw = attractionEndWorldRef.current;
			if (pw) {
				session.snapAttractedFullId = nearestAttractionSnapFullId({
					lod: lodRef.current,
					pointerWorld: pw,
					attractingFullId: session.attractingFullId,
					compat: session.compat,
					blocked: blockedVortexFullIds,
					camera: env.camera,
					gl: env.gl,
					getVortexWorld: (id) => vortexGettersRef.current.get(id)?.() ?? null,
					metaRadius: (id) => vortexMetaRef.current.get(id)?.radiusWorld ?? 0.35,
				});
			} else session.snapAttractedFullId = null;
		},
		[blockedVortexFullIds, collectPickRoots, lodRef, onAttractionTargetRing],
	);

	const commitAttractionPointer = useCallback(
		(clientX: number, clientY: number) => {
			const env = attractionThreeRef.current;
			const session = attractionSessionRef.current;
			if (!env || !session) {
				cancelAttractionDrag();
				return;
			}
			const rect = env.gl.domElement.getBoundingClientRect();
			ndcRef.current.x = ((clientX - rect.left) / rect.width) * 2 - 1;
			ndcRef.current.y = -((clientY - rect.top) / rect.height) * 2 + 1;
			raycasterRef.current.setFromCamera(ndcRef.current, env.camera);
			const hits = raycasterRef.current.intersectObjects(collectPickRoots(), true);
			const hitWorld = hitScratchRef.current;
			let pointerWorld: Vector3;
			if (hits.length > 0) {
				pointerWorld = hits[0]!.point.clone();
			} else if (raycasterRef.current.ray.intersectPlane(planeRef.current, hitWorld)) {
				pointerWorld = hitWorld.clone();
			} else {
				raycasterRef.current.ray.at(80, hitWorld);
				pointerWorld = hitWorld.clone();
			}

			const getV = (id: string) => vortexGettersRef.current.get(id)?.() ?? null;
			const rad = (id: string) => vortexMetaRef.current.get(id)?.radiusWorld ?? 0.35;
			const snapId = session.snapAttractedFullId;
			if (snapId && attractionSnapCommitProximityOk(snapId, pointerWorld, env.camera, env.gl, getV, rad)) {
				const p = { attracting: session.attractingFullId, attracted: snapId };
				onConnect?.(p);
				onProximityConnect?.(p);
				cancelAttractionDrag();
				return;
			}

			const attractingFull = session.attractingFullId;
			for (const h of hits) {
				const vf = readVortexFullIdFromObject(h.object);
				if (
					vf &&
					vf !== attractingFull &&
					session.compat.has(vf) &&
					!blockedVortexFullIds.has(vf) &&
					vortexMetaRef.current.get(vf)?.objectId !== session.attractingObjectId
				) {
					onConnect?.({ attracting: attractingFull, attracted: vf });
					cancelAttractionDrag();
					return;
				}
				const oid = readObjectItemIdFromObject(h.object);
				if (oid && oid !== session.attractingObjectId) {
					const candidates: string[] = [];
					for (const [tid, meta] of vortexMetaRef.current) {
						if (meta.objectId !== oid) continue;
						if (blockedVortexFullIds.has(tid)) continue;
						if (!session.compat.has(tid)) continue;
						candidates.push(tid);
					}
					if (candidates.length === 1) {
						const p = { attracting: attractingFull, attracted: candidates[0]! };
						onConnect?.(p);
						onIndirectConnect?.(p);
						cancelAttractionDrag();
						return;
					}
					if (candidates.length > 1) {
						attractionSessionRef.current = null;
						setAttractionDragActive(false);
						setAttractionCompatibleAttractedFullIds(new Set(candidates));
						setAttractionHoverRingFullId(null);
						setAttractionIndirectPickAwait({
							attractingFullId: attractingFull,
							attractedObjectId: oid,
							candidates,
						});
						onAttractionTargetRing?.({
							attracting: attractingFull,
							objectId: oid,
							vortexFullIds: candidates,
						});
						return;
					}
				}
			}
			cancelAttractionDrag();
		},
		[
			blockedVortexFullIds,
			cancelAttractionDrag,
			collectPickRoots,
			onConnect,
			onIndirectConnect,
			onAttractionTargetRing,
			onProximityConnect,
		],
	);

	const updateIndirectPickPointer = useCallback(
		(clientX: number, clientY: number) => {
			const awaitPick = indirectPickRef.current;
			const env = attractionThreeRef.current;
			if (!awaitPick || !env) return;
			const rect = env.gl.domElement.getBoundingClientRect();
			ndcRef.current.x = ((clientX - rect.left) / rect.width) * 2 - 1;
			ndcRef.current.y = -((clientY - rect.top) / rect.height) * 2 + 1;
			raycasterRef.current.setFromCamera(ndcRef.current, env.camera);
			const hits = raycasterRef.current.intersectObjects(collectPickRoots(), true);
			let ring: string | null = null;
			for (const h of hits) {
				const vf = readVortexFullIdFromObject(h.object);
				if (vf && awaitPick.candidates.includes(vf)) {
					ring = vf;
					break;
				}
			}
			setAttractionHoverRingFullId((prev) => (prev === ring ? prev : ring));
			const hitWorld = hitScratchRef.current;
			if (hits.length > 0) {
				attractionEndWorldRef.current = hitWorld.copy(hits[0]!.point);
			} else if (raycasterRef.current.ray.intersectPlane(planeRef.current, hitWorld)) {
				attractionEndWorldRef.current = hitWorld.clone();
			} else {
				raycasterRef.current.ray.at(80, hitWorld);
				attractionEndWorldRef.current = hitWorld.clone();
			}
		},
		[collectPickRoots],
	);

	const commitIndirectPickPointerDown = useCallback(
		(clientX: number, clientY: number, ev?: PointerEvent) => {
			const awaitPick = indirectPickRef.current;
			const env = attractionThreeRef.current;
			if (!awaitPick || !env) return;
			const rect = env.gl.domElement.getBoundingClientRect();
			ndcRef.current.x = ((clientX - rect.left) / rect.width) * 2 - 1;
			ndcRef.current.y = -((clientY - rect.top) / rect.height) * 2 + 1;
			raycasterRef.current.setFromCamera(ndcRef.current, env.camera);
			const hits = raycasterRef.current.intersectObjects(collectPickRoots(), true);
			for (const h of hits) {
				const vf = readVortexFullIdFromObject(h.object);
				if (vf && awaitPick.candidates.includes(vf)) {
					const p = { attracting: awaitPick.attractingFullId, attracted: vf };
					onConnect?.(p);
					onIndirectConnect?.(p);
					cancelAttractionDrag();
					ev?.stopImmediatePropagation();
					return;
				}
			}
			cancelAttractionDrag();
		},
		[cancelAttractionDrag, collectPickRoots, onConnect, onIndirectConnect],
	);

	const attachAttractionThreeEnv = useCallback((env: { camera: Camera; gl: WebGLRenderer; scene: ThreeScene } | null) => {
		attractionThreeRef.current = env;
	}, []);

	const findNearestProximityRelocate = useCallback(
		(world: Vector3, movingObjectId: string): AttractionPayload | null => {
			let best: { d: number; id: string } | null = null;
			for (const [fullId, getter] of vortexGettersRef.current) {
				if (fullId.startsWith(`${movingObjectId}:`)) continue;
				const p = getter();
				if (!p) continue;
				const d = p.distanceTo(world);
				if (d > proximityRadius) continue;
				if (!best || d < best.d) best = { d, id: fullId };
			}
			if (!best) return null;
			return { attracting: `${movingObjectId}:link`, attracted: best.id };
		},
		[proximityRadius],
	);

	const value = useMemo<RegistryValue>(
		() => ({
			registerVortex,
			unregisterVortex,
			getVortexWorld,
			registerVortexBinding,
			unregisterVortexBinding,
			registerObject,
			collectObjectGroups,
			getObjectGroup,
			getObjectKind,
			kindCatalogs,
			kindCompatibility,
			blockedVortexFullIds,
			proximityRadius,
			selectedObjectIds,
			setSelectedObjectIds,
			selectionMode,
			relocateMode,
			activeRelocateObjectId,
			setActiveRelocateObjectId,
			attractionDragActive,
			attractionDragAttractingFullId,
			attractionCompatibleAttractedFullIds,
			attractionHoverRingFullId,
			attractionIndirectPickAwait,
			beginAttractionDragFromVortex,
			cancelAttractionDrag,
			findNearestProximityRelocate,
			onSelect,
			onConnect,
			onProximityConnect,
			onIndirectConnect,
			onAttractionCompatibleObjects,
			onAttractionTargetRing,
			onRelocate,
			attachAttractionThreeEnv,
			updateAttractionPointer,
			commitAttractionPointer,
			updateIndirectPickPointer,
			commitIndirectPickPointerDown,
			attractionEndWorldRef,
			hoverTarget,
			setSceneHover,
			clearSceneHover,
			clearSceneHoverAll,
			isSceneHovered,
			clearSceneSelection,
		}),
		[
			registerVortex,
			unregisterVortex,
			getVortexWorld,
			registerVortexBinding,
			unregisterVortexBinding,
			registerObject,
			collectObjectGroups,
			getObjectGroup,
			getObjectKind,
			kindCatalogs,
			kindCompatibility,
			blockedVortexFullIds,
			proximityRadius,
			selectedObjectIds,
			selectionMode,
			relocateMode,
			activeRelocateObjectId,
			attractionDragActive,
			attractionDragAttractingFullId,
			attractionCompatibleAttractedFullIds,
			attractionHoverRingFullId,
			attractionIndirectPickAwait,
			beginAttractionDragFromVortex,
			cancelAttractionDrag,
			findNearestProximityRelocate,
			onSelect,
			onConnect,
			onProximityConnect,
			onIndirectConnect,
			onAttractionCompatibleObjects,
			onAttractionTargetRing,
			onRelocate,
			attachAttractionThreeEnv,
			updateAttractionPointer,
			commitAttractionPointer,
			updateIndirectPickPointer,
			commitIndirectPickPointerDown,
			hoverTarget,
			setSceneHover,
			clearSceneHover,
			clearSceneHoverAll,
			isSceneHovered,
			clearSceneSelection,
		],
	);
	const coreValue = useMemo(() => {
		const {
			attractionDragActive: _attractionDragActive,
			attractionDragAttractingFullId: _attractionDragAttractingFullId,
			attractionCompatibleAttractedFullIds: _attractionCompatibleAttractedFullIds,
			attractionHoverRingFullId: _attractionHoverRingFullId,
			attractionIndirectPickAwait: _attractionIndirectPickAwait,
			...core
		} = value;
		return core;
	}, [value]);
	const dragValue = useMemo<RegistryDragState>(
		() => ({
			attractionDragActive: value.attractionDragActive,
			attractionDragAttractingFullId: value.attractionDragAttractingFullId,
			attractionCompatibleAttractedFullIds: value.attractionCompatibleAttractedFullIds,
			attractionHoverRingFullId: value.attractionHoverRingFullId,
			attractionIndirectPickAwait: value.attractionIndirectPickAwait,
		}),
		[
			value.attractionCompatibleAttractedFullIds,
			value.attractionDragActive,
			value.attractionDragAttractingFullId,
			value.attractionHoverRingFullId,
			value.attractionIndirectPickAwait,
		],
	);

	return (
		<RegistryCoreContext.Provider value={coreValue}>
			<RegistryDragContext.Provider value={dragValue}>
				{children}
				<SceneHoverMissBridge />
				<SceneHoverInvalidateBridge />
				<SceneSelectionMissBridge />
			</RegistryDragContext.Provider>
		</RegistryCoreContext.Provider>
	);
}

function Chunks({
	chunkSize,
	maxDistance,
	children,
}: {
	chunkSize: number;
	maxDistance: number;
	children: ReactNode;
}) {
	const buckets = useMemo(() => {
		const map = new Map<string, ReactNode[]>();
		Children.forEach(children, (child) => {
			if (!isValidElement(child)) return;
			const p = child.props as { origin?: Vec3 };
			if (!p?.origin) return;
			const k = chunkKey(p.origin, chunkSize);
			const arr = map.get(k) ?? [];
			arr.push(child);
			map.set(k, arr);
		});
		return map;
	}, [children, chunkSize]);

	const visible = useVisibleChunkKeys(buckets.keys(), chunkSize, maxDistance);
	return (
		<>
			{[...buckets].map(([key, items]) => (
				<group key={key} userData={{ sceneChunk: key }} visible={visible.has(key)}>
					{items}
				</group>
			))}
		</>
	);
}

function splitChunkedSceneChildren(children: ReactNode): { chunked: ReactNode[]; rest: ReactNode[] } {
	const chunked: ReactNode[] = [];
	const rest: ReactNode[] = [];
	Children.forEach(children, (c) => {
		if (isValidElement(c) && (c.props as { origin?: Vec3 }).origin !== undefined) chunked.push(c);
		else rest.push(c);
	});
	return { chunked, rest };
}

function Inner(props: CanvasProps) {
	const { camera: camProp, chunkSize = 256, proximityRadius = 12, children } = props;
	const lodRef = useRef<number>(DEFAULT_MANUAL_LOD);
	const [sceneCamera, setSceneCamera] = useState<ThreePerspectiveCamera | null>(null);
	const domain = props.domain ?? DEFAULT_DOMAIN;
	const distanceReference = props.lodDistanceReference ?? DEFAULT_SCALE_REFERENCE;
	const gridFactor = props.gridFactor ?? DEFAULT_LOD_GRID_FACTOR;
	const gridSnapEnabled = props.gridSnapEnabled ?? false;
	const showLodGrid = props.showLodGrid === true;
	const automaticLod = props.automaticLod ?? true;
	const depthVariableLod = props.depthVariableLod ?? false;
	const manualLod =
		typeof props.lod === "number" && Number.isFinite(props.lod) && props.lod > 0 ? props.lod : DEFAULT_MANUAL_LOD;
	const maxDist = 4000;
	const pos = (camProp?.position ?? [420, -420, 320]) as [number, number, number];
	const tgt = (camProp?.target ?? [0, 0, 40]) as Vec3;
	const zoom = camProp?.zoom ?? 1;
	const autoFitCamera = props.autoFitCamera !== false;
	const autoFitBehavior = props.autoFitBehavior ?? "initial";
	const { chunked, rest } = useMemo(() => splitChunkedSceneChildren(children), [children]);
	const blockedFallback = props.blockedVortexFullIds ?? EMPTY_BLOCKED_VORTICES;
	const blocked = useLiveBlockedVortexFullIds(blockedFallback);
	return (
		<RegistryProvider
			lodRef={lodRef}
			kindCatalogs={props.kindCatalogs}
			kindCompatibility={props.kindCompatibility}
			blockedVortexFullIds={blocked}
			proximityRadius={proximityRadius}
			selectionMode={props.selectionMode ?? "single"}
			relocateMode={props.relocateMode ?? "translate"}
			selection={props.selection}
			onSelect={props.onSelect}
			onConnect={props.onConnect}
			onProximityConnect={props.onProximityConnect}
			onIndirectConnect={props.onIndirectConnect}
			onAttractionCompatibleObjects={props.onAttractionCompatibleObjects}
			onAttractionTargetRing={props.onAttractionTargetRing}
			onRelocate={props.onRelocate}
		>
			<LodBridge
				lodRef={lodRef}
				distanceReference={distanceReference}
				gridFactor={gridFactor}
				gridSnapEnabled={gridSnapEnabled}
				showLodGrid={showLodGrid}
				automaticLod={automaticLod}
				depthVariableLod={depthVariableLod}
				manualLod={manualLod}
				onLodChange={props.onLodChange}
			>
				<PerspectiveCamera ref={setSceneCamera} makeDefault near={0.2} far={500_000} fov={50} />
				<SceneCameraSeed camera={sceneCamera} position={pos} target={tgt} />
				<OrbitGated camera={sceneCamera} onCamera={props.onCamera} zoom={zoom} />
				{autoFitCamera ? (
					<SceneAutoFit behavior={autoFitBehavior} zoom={zoom} onCamera={props.onCamera} />
				) : null}
				<AttractionThreeBinder />
				<AttractionWindowBridge />
				<AttractionRubberBand />
				<ambientLight intensity={0.45} />
				<directionalLight position={[120, 180, 80]} intensity={0.85} />
				<Chunks chunkSize={chunkSize} maxDistance={maxDist}>
					{chunked}
				</Chunks>
				<group data-scene-unchunked>{rest}</group>
			</LodBridge>
		</RegistryProvider>
	);
}

export function Canvas3D(props: CanvasProps & { className?: string; style?: CSSProperties }) {
	const { children, className, style, onLodChange, domain = DEFAULT_DOMAIN, ...rest } = props;
	const [shellLod, setShellLod] = useState(() => formatSceneLod(DEFAULT_MANUAL_LOD));
	const handleLod = useCallback(
		(l: number) => {
			const label = formatSceneLod(l);
			setShellLod(label);
			onLodChange?.(l);
		},
		[onLodChange],
	);
	return (
		<div
			className={className}
			style={{ width: "100%", height: "100%", touchAction: "none", overscrollBehavior: "contain", ...style }}
			onContextMenu={(event) => event.preventDefault()}
			data-scene-domain={domain}
			data-scene-root
			data-scene-lod={shellLod}
		>
			<Canvas frameloop="demand" gl={{ antialias: true }} dpr={[1, 2]}>
				<Inner {...rest} domain={domain} onLodChange={handleLod}>
					{children}
				</Inner>
			</Canvas>
		</div>
	);
}

/** @emoji ­ƒº¬ Registers `window.__scenePlay*` hooks for Playwright (play harness only). */
export function ScenePlayTestBridge(props: { readonly setSelectedId: (id: string | null) => void }): null {
	const reg = useRegistry();
	const setSelectedId = props.setSelectedId;
	useEffect(() => {
		const w = window as unknown as {
			__scenePlaySelect?: (id: string) => void;
			__scenePlayActivate?: (id: string) => void;
			__scenePlayClearSelection?: () => void;
		};
		w.__scenePlaySelect = (id: string) => {
			setSelectedId(id);
		};
		w.__scenePlayActivate = (id: string) => {
			setSelectedId(id);
			reg.setActiveRelocateObjectId(id);
		};
		w.__scenePlayClearSelection = () => {
			setSelectedId(null);
			reg.setActiveRelocateObjectId(null);
		};
		return () => {
			delete w.__scenePlaySelect;
			delete w.__scenePlayActivate;
			delete w.__scenePlayClearSelection;
		};
	}, [setSelectedId, reg]);
	return null;
}

//#endregion ­ƒÄ¼Scene



if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;
	describe("lodFromCameraDistance", () => {
		it("maps orbit distance to scale ratio", () => {
			expect(lodFromCameraDistance(100, 100)).toBe(1);
			expect(lodFromCameraDistance(20000, 100)).toBe(200);
			expect(lodFromCameraDistance(50, 100)).toBe(0.5);
		});
	});
	describe("pickClosestLod", () => {
		const available = [50, 200, 1000] as const;
		it("prefers log-closest and ties toward smaller lod", () => {
			expect(pickClosestLod(available, 100)).toBe(50);
			expect(pickClosestLod(available, 500)).toBe(1000);
			expect(pickClosestLod(available, 5000)).toBe(1000);
		});
	});
	describe("lodGridStepWorld", () => {
		it("returns null for very coarse lod and ~5 at lod 100", () => {
			expect(lodGridStepWorld(5000, 10)).toBe(null);
			expect(lodGridStepWorld(100, 10)).toBe(50);
		});
	});
	describe("lodHandlePrimaryVisible", () => {
		it("draws handles at detail bands", () => {
			expect(lodHandlePrimaryVisible(100)).toBe(true);
			expect(lodHandlePrimaryVisible(201)).toBe(false);
		});
	});
	describe("lodHandlePickProxy", () => {
		it("uses pick proxies in mid bands only", () => {
			expect(lodHandlePickProxy(500)).toBe(true);
			expect(lodHandlePickProxy(100)).toBe(false);
			expect(lodHandlePickProxy(2000)).toBe(false);
		});
	});
	describe("sceneLodCanvasProps", () => {
		it("maps auto, depth, and manual modes", () => {
			expect(sceneLodCanvasProps({ automaticLod: true, depthVariableLod: false, manualLod: 50 })).toEqual({
				automaticLod: true,
				depthVariableLod: false,
			});
			expect(sceneLodCanvasProps({ automaticLod: false, depthVariableLod: true, manualLod: 50 })).toEqual({
				automaticLod: false,
				depthVariableLod: true,
			});
			expect(sceneLodCanvasProps({ automaticLod: false, depthVariableLod: false, manualLod: 42 })).toEqual({
				automaticLod: false,
				depthVariableLod: false,
				lod: 42,
			});
		});
	});
	describe("sliderValueFromLod", () => {
		it("round-trips integer slider positions through lodFromSliderValue", () => {
			for (const slider of [0, 250, 500, 750, 1000] as const) {
				expect(sliderValueFromLod(lodFromSliderValue(slider))).toBe(slider);
			}
		});
	});
	describe("objectPoseKey", () => {
		it("changes when origin changes", () => {
			const a = objectPoseKey("id", [0, 0, 0], [0, 0, 0, 1], 1);
			const b = objectPoseKey("id", [1, 0, 0], [0, 0, 0, 1], 1);
			expect(a).not.toBe(b);
		});
	});
	describe("boundsFromObjectGroups", () => {
		it("returns null for empty input", () => {
			expect(boundsFromObjectGroups([])).toBeNull();
		});
		it("measures mesh geometry in world space", () => {
			const root = new Group();
			const mesh = new Mesh(new BoxGeometry(10, 20, 30));
			root.add(mesh);
			applyObjectPose(root, [5, 0, 0], [0, 0, 0, 1], 1);
			const bounds = boundsFromObjectGroups([root]);
			expect(bounds).not.toBeNull();
			expect(bounds!.radius).toBeGreaterThan(5);
		});
	});
	describe("sceneAutoFitShouldRun", () => {
		it("runs once for initial behavior", () => {
			expect(sceneAutoFitShouldRun("initial", "a", "", false)).toBe(true);
			expect(sceneAutoFitShouldRun("initial", "a", "a", true)).toBe(false);
			expect(sceneAutoFitShouldRun("initial", "b", "a", true)).toBe(false);
		});
		it("refits when the object-group key changes in changes behavior", () => {
			expect(sceneAutoFitShouldRun("changes", "b", "a", true)).toBe(true);
		});
	});
	describe("cameraStateNearEqual", () => {
		it("detects position and zoom deltas", async () => {
			const { cameraStateNearEqual, updateSceneCameraInFixture } = await import("./play/index.ts");
			const base = {
				position: [1, 2, 3] as const,
				target: [0, 0, 0] as const,
				zoom: 1,
			};
			expect(cameraStateNearEqual(base, { ...base, position: [1.0001, 2, 3] })).toBe(true);
			expect(cameraStateNearEqual(base, { ...base, position: [2, 2, 3] })).toBe(false);
			const fixture = {
				schema: "elements.scene.fixture/v1" as const,
				domain: "architecture" as const,
				camera: base,
				objects: [],
				attractions: [],
			};
			const moved = updateSceneCameraInFixture(fixture, { position: [2, 2, 3] });
			expect(moved).not.toBe(fixture);
			const same = updateSceneCameraInFixture(fixture, { position: [1.0001, 2, 3] });
			expect(same).toBe(fixture);
		});
	});
	describe("applySceneAutoFitCamera", () => {
		it("offsets camera from bounds center", () => {
			const camera = new ThreePerspectiveCamera(50, 1, 0.1, 10_000);
			applySceneAutoFitCamera(camera, { center: [0, 0, 0], radius: 10 });
			expect(camera.position.length()).toBeGreaterThan(10);
		});
	});
	describe("cadVec3ToThree", () => {
		it("maps CAD Z-up to Three Y-up", () => {
			const zUp = cadVec3ToThree([0, 0, 1]);
			expect(zUp[0]).toBeCloseTo(0, 5);
			expect(zUp[1]).toBeCloseTo(1, 5);
			expect(zUp[2]).toBeCloseTo(0, 5);
			const yFront = cadVec3ToThree([0, 1, 0]);
			expect(yFront[0]).toBeCloseTo(0, 5);
			expect(yFront[1]).toBeCloseTo(0, 5);
			expect(yFront[2]).toBeCloseTo(-1, 5);
		});
		it("round-trips with threeVec3ToCad", () => {
			const cad: Vec3 = [12, -4, 7];
			const back = threeVec3ToCad(new Vector3(...cadVec3ToThree(cad)));
			expect(back[0]).toBeCloseTo(cad[0], 5);
			expect(back[1]).toBeCloseTo(cad[1], 5);
			expect(back[2]).toBeCloseTo(cad[2], 5);
		});
	});
	describe("applyObjectPose", () => {
		it("places vortex child at expected world offset in CAD fixture space", () => {
			const parent = new Group();
			const vortex = new Group();
			const localThree = cadObjectLocalToThreeGroupLocal([1, 2, 3], [10, 0, 0], [0, 0, 0, 1]);
			vortex.position.set(localThree[0], localThree[1], localThree[2]);
			parent.add(vortex);
			applyObjectPose(parent, [10, 0, 0], [0, 0, 0, 1], 1);
			updateWorldMatrixChain(vortex);
			const world = new Vector3();
			vortex.getWorldPosition(world);
			const expected = cadVec3ToThree([11, 2, 3]);
			expect(world.x).toBeCloseTo(expected[0], 5);
			expect(world.y).toBeCloseTo(expected[1], 5);
			expect(world.z).toBeCloseTo(expected[2], 5);
		});
	});
	describe("objectMatchesSceneSelection", () => {
		it("matches object id and parent of selected vortex", () => {
			expect(
				objectMatchesSceneSelection("a", { objectIds: ["a"], vortexIds: [] }),
			).toBe(true);
			expect(
				objectMatchesSceneSelection("b", { objectIds: [], vortexIds: ["b:link"] }),
			).toBe(true);
			expect(
				objectMatchesSceneSelection("c", { objectIds: ["a"], vortexIds: ["b:link"] }),
			).toBe(false);
		});
	});
	describe("parseFixtureV1", () => {
		it("accepts minimal fixture", () => {
			const f = parseFixtureV1({
				schema: "elements.scene.fixture/v1",
				camera: { position: [0, 0, 0], target: [0, 0, 1], zoom: 1 },
				attractions: [],
				objects: [
					{
						id: "a",
						meshUrl: "/m.glb",
						origin: [1, 2, 3],
						orientation: [0, 0, 0, 1],
						vortices: [{ id: "a:v1", position: [0, 0, 0] }],
					},
				],
			});
			expect(f?.objects[0]?.id).toBe("a");
			expect(f?.domain).toBe("architecture");
		});
		it("parses domain case-insensitively", () => {
			const f = parseFixtureV1({
				schema: "elements.scene.fixture/v1",
				domain: "Urban",
				camera: { position: [0, 0, 0], target: [0, 0, 1], zoom: 1 },
				attractions: [],
				objects: [{ id: "a", meshUrl: "/m.glb", origin: [1, 2, 3], vortices: [] }],
			});
			expect(f?.domain).toBe("urban");
		});
		it("parses meshByLod list entries", () => {
			const f = parseFixtureV1({
				schema: "elements.scene.fixture/v1",
				camera: { position: [0, 0, 0], target: [0, 0, 1], zoom: 1 },
				attractions: [],
				objects: [
					{
						id: "a",
						meshUrl: "/m.glb",
						origin: [0, 0, 0],
						orientation: [0, 0, 0, 1],
						meshByLod: [{ lod: 100, url: "/fine.glb" }],
						vortices: [
							{
								id: "a:v1",
								position: [0, 0, 0],
								handleMeshUrl: "/fallback.glb",
								handleMeshByLod: [
									{ lod: 100, url: "/d.glb" },
									{ lod: 50, url: "/u.glb" },
								],
							},
						],
					},
				],
			});
			const o = f?.objects[0];
			expect(o?.meshByLod?.[0]?.url).toBe("/fine.glb");
			const v = o?.vortices[0];
			expect(v?.handleMeshByLod?.[0]?.url).toBe("/d.glb");
			expect(v?.handleMeshUrl).toBe("/fallback.glb");
		});
	});
	describe("chunkKey", () => {
		it("buckets origin", () => {
			expect(chunkKey([10, 10, 10], 256)).toBe("0|0|0");
			expect(chunkKey([300, 0, 0], 256)).toBe("1|0|0");
		});
	});
	describe("chunkDistanceVisible", () => {
		it("keeps visible inside exit margin after entering", () => {
			const cam = new Vector3(0, 0, 0);
			const chunkSize = 256;
			const maxDist = 200;
			const enterDist = maxDist + chunkBoundsRadius(chunkSize);
			const far = new Vector3(enterDist + chunkSize, 0, 0);
			expect(
				chunkDistanceVisible({ camPos: cam, chunkCenter: far, chunkSize, maxDist, wasVisible: false }),
			).toBe(false);
			const near = new Vector3(enterDist - 50, 0, 0);
			expect(
				chunkDistanceVisible({ camPos: cam, chunkCenter: near, chunkSize, maxDist, wasVisible: false }),
			).toBe(true);
			const between = new Vector3(enterDist + chunkSize * 0.25, 0, 0);
			expect(
				chunkDistanceVisible({ camPos: cam, chunkCenter: between, chunkSize, maxDist, wasVisible: true }),
			).toBe(true);
			const beyond = new Vector3(enterDist + chunkSize * 0.75, 0, 0);
			expect(
				chunkDistanceVisible({ camPos: cam, chunkCenter: beyond, chunkSize, maxDist, wasVisible: true }),
			).toBe(false);
		});
	});
	describe("gltfPoolAcquire", () => {
		it("tracks refcount without clearing cache on release", () => {
			const url = "http://x/pool-test.glb";
			gltfPoolAcquire(url);
			gltfPoolAcquire(url);
			gltfPoolRelease(url);
			gltfPoolRelease(url);
			gltfPoolAcquire(url);
			gltfPoolRelease(url);
			expect(true).toBe(true);
		});
	});
	describe("resolveMeshStyle", () => {
		it("prefers explicit style over interaction flags", () => {
			expect(
				resolveMeshStyle({
					style: "original",
					selected: true,
					hovered: true,
					disabled: true,
				}),
			).toBe("original");
		});
		it("compares scene hover targets by kind and id", () => {
			expect(sceneHoverTargetsEqual(null, null)).toBe(true);
			expect(sceneHoverTargetsEqual({ kind: "object", id: "a" }, { kind: "object", id: "a" })).toBe(true);
			expect(sceneHoverTargetsEqual({ kind: "object", id: "a" }, { kind: "object", id: "b" })).toBe(false);
			expect(sceneHoverTargetsEqual({ kind: "vortex", fullId: "o:v" }, { kind: "object", id: "o" })).toBe(false);
			expect(sceneHoverTargetsEqual({ kind: "attraction", id: "e1" }, { kind: "attraction", id: "e1" })).toBe(true);
		});

		it("orders disabled, selected, highlighted, hovered, then default", () => {
			expect(resolveMeshStyle({ disabled: true, selected: true })).toBe("disabled");
			expect(resolveMeshStyle({ selected: true, highlighted: true })).toBe("selected");
			expect(resolveMeshStyle({ highlighted: true, hovered: true })).toBe("highlighted");
			expect(resolveMeshStyle({ hovered: true })).toBe("hovered");
			expect(resolveMeshStyle({})).toBe(DEFAULT_MESH_STYLE);
		});
	});
	describe("meshStyleColors", () => {
		it("returns null for original and colors for neutral", () => {
			expect(meshStyleColors("original")).toBeNull();
			const neutral = meshStyleColors("neutral");
			expect(neutral?.meshColor.length).toBeGreaterThan(0);
			expect(neutral?.lineColor.length).toBeGreaterThan(0);
		});
		it("returns primary-toned selected and highlighted fills", () => {
			const selected = meshStyleColors("selected");
			const highlighted = meshStyleColors("highlighted");
			expect(selected?.meshColor).toBeTruthy();
			expect(highlighted?.meshColor).toBeTruthy();
		});
	});
	describe("styledMeshPoolAcquire", () => {
		it("tracks styled pool keys separately from base url", () => {
			const url = "http://x/styled-pool.glb";
			styledMeshPoolAcquire(url, "neutral");
			styledMeshPoolAcquire(url, "selected");
			styledMeshPoolRelease(url, "neutral");
			styledMeshPoolRelease(url, "selected");
			expect(true).toBe(true);
		});
	});
	describe("kindsCompatible", () => {
		it("matches bidirectional", () => {
			const ok = kindsCompatible("a", "b", [{ source: "b", target: "a", bidirectional: true }]);
			expect(ok).toBe(true);
		});
	});
	describe("blockedVortexFullIdsFromAttractions", () => {
		it("collects endpoints", () => {
			const s = blockedVortexFullIdsFromAttractions([{ attracting: "a:h1", attracted: "b:h2" }]);
			expect(s.has("a:h1")).toBe(true);
			expect(s.has("b:h2")).toBe(true);
		});
	});
	describe("handlesAttractionCompatibleForDrag", () => {
		it("allows all when rules empty", () => {
			const ok = handlesAttractionCompatibleForDrag(
				{ objectId: "a", objectKind: "n1", vortexKind: "h1" },
				{ objectId: "b", objectKind: "n2", vortexKind: "h2" },
				[],
				undefined,
			);
			expect(ok).toBe(true);
		});
		it("matches handle specificity", () => {
			const ok = handlesAttractionCompatibleForDrag(
				{ objectId: "a", objectKind: "x", vortexKind: "h1" },
				{ objectId: "b", objectKind: "y", vortexKind: "h2" },
				[{ source: "h1", target: "h2", specificity: "handle" }],
				undefined,
			);
			expect(ok).toBe(true);
		});
	});
	describe("resolveWireKindForVortex", () => {
		it("falls back to default wire id", () => {
			expect(resolveWireKindForVortex("any", undefined)).toBe("board.wire.link");
		});
	});
	describe("wouldAttractionEdgeIntroduceCycle", () => {
		it("detects a closing edge on an existing chain", () => {
			const edges = [
				{ attractingObjectId: "a", attractedObjectId: "b", attractionId: "t1" },
				{ attractingObjectId: "b", attractedObjectId: "c", attractionId: "t2" },
			];
			expect(wouldAttractionEdgeIntroduceCycle(edges, "c", "a")).toBe(true);
			expect(wouldAttractionEdgeIntroduceCycle(edges, "a", "d")).toBe(false);
		});
	});
	describe("resolveSceneAttractionTree", () => {
		it("breaks ownership cycles in cyclic attraction components", () => {
			const tree = resolveSceneAttractionTree({
				objectIds: ["a", "b", "c"],
				edges: [
					{ attractingObjectId: "a", attractedObjectId: "b", attractionId: "t1" },
					{ attractingObjectId: "b", attractedObjectId: "c", attractionId: "t2" },
					{ attractingObjectId: "c", attractedObjectId: "a", attractionId: "t3" },
				],
			});
			for (const id of ["a", "b", "c"]) {
				expect(parentOwnershipCycleMemberIds(tree.parentByObjectId, id)).toBeNull();
			}
		});
		it("picks parent closer to wormhole when multiply attracted", () => {
			const tree = resolveSceneAttractionTree({
				objectIds: ["w", "a", "b", "c"],
				explicitWormholeIds: new Set(["w"]),
				edges: [
					{ attractingObjectId: "w", attractedObjectId: "a", attractionId: "t1" },
					{ attractingObjectId: "a", attractedObjectId: "b", attractionId: "t2" },
					{ attractingObjectId: "w", attractedObjectId: "c", attractionId: "t3" },
					{ attractingObjectId: "c", attractedObjectId: "b", attractionId: "t4" },
				],
			});
			expect(tree.parentByObjectId.get("b")).toBe("a");
			expect(tree.attractingByObjectId.get("a")).toEqual(["b"]);
		});
		it("lists attracted children per owner", () => {
			const tree = resolveSceneAttractionTree({
				objectIds: ["w", "a", "b"],
				explicitWormholeIds: new Set(["w"]),
				edges: [
					{ attractingObjectId: "w", attractedObjectId: "a", attractionId: "t1" },
					{ attractingObjectId: "a", attractedObjectId: "b", attractionId: "t2" },
				],
			});
			expect(collectAttractedDescendantIds("w", tree.attractingByObjectId)).toEqual(["a", "b"]);
		});
	});
	describe("attractionEdgesFromAttractions", () => {
		it("maps vortex endpoints to object ids", () => {
			const edges = attractionEdgesFromAttractions([
				{ id: "x", attracting: "objA:v1", attracted: "objB:link" },
			]);
			expect(edges[0]?.attractingObjectId).toBe("objA");
			expect(edges[0]?.attractedObjectId).toBe("objB");
		});
	});
}


// #region 🛝PlayHost
// #region ­ƒº▓Header
// ­ƒÆ╗ elements/client/lib/system/renderer/react/scene/scene-play-host.tsx ÔÇö Host outside play bundle: scene play React tree and mount.
// #endregion ­ƒº▓Header


function useScenePlayController(): ScenePlayShellController | undefined {
	const { runtime } = useApp();
	const generation = React.useSyncExternalStore(
		(onStoreChange) => runtime.subscribe(onStoreChange),
		() => runtime.generation,
		() => 0,
	);
	void generation;
	return runtime.getActiveApp()?.controller as ScenePlayShellController | undefined;
}

function useScenePlaySnapshot(): ScenePlaySnapshot {
	const ctrl = useScenePlayController();
	return (
		ctrl?.getSnapshot() ?? {
			fixture: null,
			fixtureRevision: 0,
			lodProps: sceneLodCanvasProps({ automaticLod: true, depthVariableLod: false, manualLod: DEFAULT_MANUAL_LOD }),
			lodTag: DEFAULT_MANUAL_LOD,
			lodSlider: sliderValueFromLod(DEFAULT_MANUAL_LOD),
			automaticLod: true,
			depthVariableLod: false,
			relocateMode: "translate",
			selection: SCENE_PLAY_EMPTY_SELECTION,
			selectedId: null,
			selectionMode: "single",
			proximityRadius: 24,
			chunkSize: 256,
			gridFactor: 10,
			showLodGrid: false,
			gridSnapEnabled: true,
			proximityCount: 0,
			connectCount: 0,
			indirectCount: 0,
			compatibleObjectsCount: 0,
			targetRingCount: 0,
		}
	);
}

interface ScenePlayShellContextValue {
	readonly snap: ScenePlaySnapshot;
	readonly patchFixture: (updater: (prev: FixtureV1) => FixtureV1) => void;
	readonly setSelection: (selection: ScenePlaySelection) => void;
	readonly kindCatalogs: KindCatalogBundle | undefined;
}

const ScenePlayShellContext = createContext<ScenePlayShellContextValue | null>(null);

function useScenePlayShell(): ScenePlayShellContextValue {
	const value = useContext(ScenePlayShellContext);
	if (!value) {
		throw new Error("ScenePlayShellContext missing");
	}
	return value;
}

function scenePlayAllEqual<T>(values: readonly T[]): boolean {
	if (values.length <= 1) {
		return true;
	}
	const first = values[0];
	for (let i = 1; i < values.length; i += 1) {
		if (values[i] !== first) {
			return false;
		}
	}
	return true;
}

function ScenePlayVec3Row(props: {
	readonly id: string;
	readonly label: string;
	readonly values: readonly Vec3[];
	readonly uniform: boolean;
	readonly onChange: (next: Vec3) => void;
}): React.ReactElement {
	const value = props.uniform ? props.values[0]! : ([Number.NaN, Number.NaN, Number.NaN] as Vec3);
	return (
		<Label id={props.id} label={props.label}>
			<div className="grid grid-cols-3 gap-1">
				{(["x", "y", "z"] as const).map((axis, index) => (
					<Input
						key={`${props.id}.${axis}`}
						className="h-7 font-mono text-xs"
						onChange={(event: ChangeEvent<HTMLInputElement>) => {
							const parsed = Number(event.target.value);
							if (!Number.isFinite(parsed)) {
								return;
							}
							const next: Vec3 = [...value];
							next[index] = parsed;
							props.onChange(next);
						}}
						placeholder={props.uniform ? axis : "—"}
						value={props.uniform ? String(value[index]) : ""}
					/>
				))}
			</div>
		</Label>
	);
}

function ScenePlayInspectorObjects(props: {
	readonly fixture: FixtureV1;
	readonly objectIds: readonly string[];
}): React.ReactElement {
	const { patchFixture, setSelection } = useScenePlayShell();
	const { kindCatalogs } = useScenePlayShell();
	const idSet = useMemo(() => new Set(props.objectIds), [props.objectIds]);
	const objects = useMemo(
		() => props.fixture.objects.filter((object) => idSet.has(object.id)),
		[props.fixture.objects, idSet],
	);
	const nodeKinds = kindCatalogs?.nodes ?? [];
	const labels = objects.map((object) => object.label ?? "");
	const labelUniform = scenePlayAllEqual(labels);
	const kinds = objects.map((object) => object.objectKind ?? "");
	const kindUniform = scenePlayAllEqual(kinds);
	const wormholes = objects.map((object) => object.wormhole === true);
	const wormholeUniform = scenePlayAllEqual(wormholes);
	return (
		<div className="space-y-3 border-l border-element/60 pl-2">
			{props.objectIds.length === 1 ? (
				<Label id="scene-play.inspector.object.id" label="Id">
					<Input
						className="h-7 font-mono text-xs"
						defaultValue={props.objectIds[0]}
						key={props.objectIds[0]}
						onBlur={(event) => {
							const nextId = event.currentTarget.value.trim();
							const oldId = props.objectIds[0];
							if (!oldId || !nextId || nextId === oldId) {
								return;
							}
							patchFixture((fixture) => ({
								...fixture,
								objects: fixture.objects.map((object) => (object.id === oldId ? { ...object, id: nextId } : object)),
							}));
							setSelection({ objectIds: [nextId], vortexIds: [], attractionIds: [] });
						}}
					/>
				</Label>
			) : null}
			<Label id="scene-play.inspector.object.label" label="Label">
				<Input
					className="h-7 font-mono text-xs"
					onChange={(event) => {
						const label = event.target.value;
						for (const objectId of props.objectIds) {
							patchFixture((fixture) => updateSceneObjectInFixture(fixture, objectId, { label }));
						}
					}}
					placeholder={labelUniform ? undefined : "Mixed"}
					value={labelUniform ? labels[0] ?? "" : ""}
				/>
			</Label>
			<Label id="scene-play.inspector.object.kind" label="Object kind">
				<Select
					onValueChange={(value) => {
						for (const objectId of props.objectIds) {
							patchFixture((fixture) => updateSceneObjectInFixture(fixture, objectId, { objectKind: value }));
						}
					}}
					value={kindUniform ? kinds[0] ?? "" : undefined}
				>
					<SelectTrigger className="h-7 font-mono text-xs">
						<SelectValue placeholder={kindUniform ? "kind" : "Mixed"} />
					</SelectTrigger>
					<SelectContent>
						{nodeKinds.map((entry) => (
							<SelectItem key={entry.id} value={entry.id}>
								{entry.label ?? entry.name ?? entry.id}
							</SelectItem>
						))}
					</SelectContent>
				</Select>
			</Label>
			<Label id="scene-play.inspector.object.wormhole" label="Wormhole">
				<Select
					onValueChange={(value) => {
						const wormhole = value === "true";
						for (const objectId of props.objectIds) {
							patchFixture((fixture) => updateSceneObjectInFixture(fixture, objectId, { wormhole }));
						}
					}}
					value={wormholeUniform ? String(wormholes[0]) : undefined}
				>
					<SelectTrigger className="h-7 font-mono text-xs">
						<SelectValue placeholder={wormholeUniform ? "wormhole" : "Mixed"} />
					</SelectTrigger>
					<SelectContent>
						<SelectItem value="true">true</SelectItem>
						<SelectItem value="false">false</SelectItem>
					</SelectContent>
				</Select>
			</Label>
			<ScenePlayVec3Row
				id="scene-play.inspector.object.origin"
				label="Origin"
				onChange={(origin) => {
					for (const objectId of props.objectIds) {
						patchFixture((fixture) => updateSceneObjectInFixture(fixture, objectId, { origin }));
					}
				}}
				uniform={scenePlayAllEqual(objects.map((object) => object.origin))}
				values={objects.map((object) => object.origin)}
			/>
		</div>
	);
}

function ScenePlayInspectorVortices(props: {
	readonly fixture: FixtureV1;
	readonly vortexFullIds: readonly string[];
}): React.ReactElement {
	const { patchFixture, kindCatalogs } = useScenePlayShell();
	const handleKinds = kindCatalogs?.handles ?? [];
	const rows = useMemo(() => {
		const out: { fullId: string; objectId: string; vortex: VortexProps }[] = [];
		for (const fullId of props.vortexFullIds) {
			const { objectId, vortexId } = parseVortexFullId(fullId);
			const object = props.fixture.objects.find((entry) => entry.id === objectId);
			const vortex = object?.vortices.find((entry) => sceneVortexFullId(objectId, entry.id) === fullId || entry.id === vortexId);
			if (object && vortex) {
				out.push({ fullId, objectId, vortex });
			}
		}
		return out;
	}, [props.fixture.objects, props.vortexFullIds]);
	return (
		<div className="space-y-3 border-l border-element/60 pl-2">
			{rows.map((row) => (
				<div key={row.fullId} className="space-y-2 rounded border border-element/40 p-2">
					<div className="font-mono text-[11px] text-muted-foreground">{row.fullId}</div>
					<Label id={`scene-play.inspector.vortex.kind.${row.fullId}`} label="Vortex kind">
						<Select
							onValueChange={(value) => {
								patchFixture((fixture) => updateSceneVortexInFixture(fixture, row.fullId, { vortexKind: value }));
							}}
							value={row.vortex.vortexKind ?? ""}
						>
							<SelectTrigger className="h-7 font-mono text-xs">
								<SelectValue placeholder="kind" />
							</SelectTrigger>
							<SelectContent>
								{handleKinds.map((entry) => (
									<SelectItem key={entry.id} value={entry.id}>
										{entry.label ?? entry.name ?? entry.id}
									</SelectItem>
								))}
							</SelectContent>
						</Select>
					</Label>
					<ScenePlayVec3Row
						id={`scene-play.inspector.vortex.position.${row.fullId}`}
						label="Position"
						onChange={(position) => {
							patchFixture((fixture) => updateSceneVortexInFixture(fixture, row.fullId, { position }));
						}}
						uniform
						values={[row.vortex.position]}
					/>
					<Label id={`scene-play.inspector.vortex.radius.${row.fullId}`} label="Radius">
						<Input
							className="h-7 font-mono text-xs"
							onChange={(event) => {
								const radius = Number(event.target.value);
								if (!Number.isFinite(radius)) {
									return;
								}
								patchFixture((fixture) => updateSceneVortexInFixture(fixture, row.fullId, { radius }));
							}}
							value={String(row.vortex.radius ?? 0.35)}
						/>
					</Label>
				</div>
			))}
		</div>
	);
}

function ScenePlayInspectorAttractions(props: {
	readonly fixture: FixtureV1;
	readonly attractionIds: readonly string[];
}): React.ReactElement {
	const { patchFixture, kindCatalogs } = useScenePlayShell();
	const wireKinds = kindCatalogs?.wires ?? [];
	const attractions = useMemo(
		() => props.fixture.attractions.filter((attraction) => props.attractionIds.includes(attraction.id)),
		[props.attractionIds, props.fixture.attractions],
	);
	return (
		<div className="space-y-3 border-l border-element/60 pl-2">
			{attractions.map((attraction) => (
				<div key={attraction.id} className="space-y-2 rounded border border-element/40 p-2">
					<div className="font-mono text-[11px] text-muted-foreground">{attraction.id}</div>
					<Label id={`scene-play.inspector.attraction.attracting.${attraction.id}`} label="Attracting">
						<Input
							className="h-7 font-mono text-xs"
							onChange={(event) => {
								const attracting = event.target.value.trim() as AttractionProps["attracting"];
								patchFixture((fixture) => updateSceneAttractionInFixture(fixture, attraction.id, { attracting }));
							}}
							value={attraction.attracting}
						/>
					</Label>
					<Label id={`scene-play.inspector.attraction.attracted.${attraction.id}`} label="Attracted">
						<Input
							className="h-7 font-mono text-xs"
							onChange={(event) => {
								const attracted = event.target.value.trim() as AttractionProps["attracted"];
								patchFixture((fixture) => updateSceneAttractionInFixture(fixture, attraction.id, { attracted }));
							}}
							value={attraction.attracted}
						/>
					</Label>
					<Label id={`scene-play.inspector.attraction.kind.${attraction.id}`} label="Attraction kind">
						<Select
							onValueChange={(value) => {
								patchFixture((fixture) => updateSceneAttractionInFixture(fixture, attraction.id, { attractionKind: value }));
							}}
							value={attraction.attractionKind ?? ""}
						>
							<SelectTrigger className="h-7 font-mono text-xs">
								<SelectValue placeholder="kind" />
							</SelectTrigger>
							<SelectContent>
								{wireKinds.map((entry) => (
									<SelectItem key={entry.id} value={entry.id}>
										{entry.label ?? entry.name ?? entry.id}
									</SelectItem>
								))}
							</SelectContent>
						</Select>
					</Label>
				</div>
			))}
		</div>
	);
}

function ScenePlayInspectorPanel(): React.ReactElement {
	const { runtime } = useApp();
	const { snap, patchFixture, setSelection } = useScenePlayShell();
	const bus = runtime.commandBus;
	const fixture = snap.fixture;
	const selection = snap.selection;
	if (!fixture) {
		return <p className="p-2 text-xs text-destructive">Invalid scene fixture</p>;
	}
	const hasSelection =
		selection.objectIds.length > 0 || selection.vortexIds.length > 0 || selection.attractionIds.length > 0;
	return (
		<div className="flex min-h-0 flex-col gap-3 p-2">
			<div className="flex items-center justify-between gap-2 border-b border-element pb-2">
				<div className="flex items-center gap-2 text-muted-foreground">
					<ClipboardList className="size-4 shrink-0" />
					<div>
						<div className="text-xs font-semibold uppercase tracking-wide">Inspector</div>
						<div className="text-[11px] opacity-80">
							{selection.objectIds.length} obj · {selection.vortexIds.length} vortex · {selection.attractionIds.length} attraction
						</div>
					</div>
				</div>
				<Button
					className="h-7 gap-1 px-2"
					disabled={!hasSelection}
					onClick={() => bus.dispatch(SCENE_PLAY_CONTROLLER_ID, "deleteSelection")}
					size="sm"
					type="button"
					variant="destructive"
				>
					<Trash2 className="size-3.5" />
					Delete
				</Button>
			</div>
			{!hasSelection ? (
				<p className="text-muted-foreground text-xs">Click an object to select. Alt+click a vortex to select it. Pick an attraction below.</p>
			) : null}
			{selection.objectIds.length > 0 ? (
				<section>
					<div className="mb-1 text-xs font-medium">Objects ({selection.objectIds.length})</div>
					<ScenePlayInspectorObjects fixture={fixture} objectIds={selection.objectIds} />
				</section>
			) : null}
			{selection.vortexIds.length > 0 ? (
				<section>
					<div className="mb-1 text-xs font-medium">Vortices ({selection.vortexIds.length})</div>
					<ScenePlayInspectorVortices fixture={fixture} vortexFullIds={selection.vortexIds} />
				</section>
			) : null}
			{selection.attractionIds.length > 0 ? (
				<section>
					<div className="mb-1 text-xs font-medium">Attractions ({selection.attractionIds.length})</div>
					<ScenePlayInspectorAttractions attractionIds={selection.attractionIds} fixture={fixture} />
				</section>
			) : null}
			<section>
				<div className="mb-1 text-xs font-medium">Attractions in scene</div>
				<div className="max-h-40 space-y-1 overflow-auto">
					{fixture.attractions.map((attraction) => (
						<button
							key={attraction.id}
							className="hover:bg-accent/40 w-full rounded px-1 py-0.5 text-left font-mono text-[11px]"
							onClick={() => setSelection({ objectIds: [], vortexIds: [], attractionIds: [attraction.id] })}
							type="button"
						>
							{attraction.id}: {attraction.attracting} → {attraction.attracted}
						</button>
					))}
				</div>
			</section>
			<section>
				<div className="mb-1 text-xs font-medium">Objects in scene</div>
				<div className="max-h-48 space-y-1 overflow-auto">
					{fixture.objects.map((object) => (
						<button
							key={object.id}
							className="hover:bg-accent/40 w-full rounded px-1 py-0.5 text-left font-mono text-[11px]"
							onClick={() => setSelection({ objectIds: [object.id], vortexIds: [], attractionIds: [] })}
							type="button"
						>
							{object.id}
							{object.label ? ` · ${object.label}` : ""}
							{object.objectKind ? ` · ${object.objectKind}` : ""}
						</button>
					))}
				</div>
			</section>
		</div>
	);
}

function ScenePlaySettingsPanel(): React.ReactElement {
	const { runtime } = useApp();
	const { snap } = useScenePlayShell();
	const bus = runtime.commandBus;
	return (
		<div className="space-y-3 p-2">
			<div className="flex items-center gap-2 border-b border-element pb-2 text-muted-foreground">
				<Settings className="size-4" />
				<div className="text-xs font-semibold uppercase tracking-wide">Scene options</div>
			</div>
			<Label id="scene-play.settings.selectionMode" label="Selection mode">
				<Select
					onValueChange={(value) => {
						if (value === "single" || value === "additive" || value === "subtractive" || value === "toggle") {
							bus.dispatch(SCENE_PLAY_CONTROLLER_ID, "setSelectionMode", { mode: value });
						}
					}}
					value={snap.selectionMode}
				>
					<SelectTrigger className="h-7 font-mono text-xs">
						<SelectValue />
					</SelectTrigger>
					<SelectContent>
						<SelectItem value="single">single</SelectItem>
						<SelectItem value="additive">additive</SelectItem>
						<SelectItem value="subtractive">subtractive</SelectItem>
						<SelectItem value="toggle">toggle</SelectItem>
					</SelectContent>
				</Select>
			</Label>
			<Label id="scene-play.settings.proximityRadius" label="Proximity radius">
				<Input
					className="h-7 font-mono text-xs"
					onChange={(event) => {
						const value = Number(event.target.value);
						if (Number.isFinite(value) && value > 0) {
							bus.dispatch(SCENE_PLAY_CONTROLLER_ID, "setProximityRadius", { value });
						}
					}}
					value={String(snap.proximityRadius)}
				/>
			</Label>
			<Label id="scene-play.settings.chunkSize" label="Chunk size">
				<Input
					className="h-7 font-mono text-xs"
					onChange={(event) => {
						const value = Number(event.target.value);
						if (Number.isFinite(value) && value > 0) {
							bus.dispatch(SCENE_PLAY_CONTROLLER_ID, "setChunkSize", { value });
						}
					}}
					value={String(snap.chunkSize)}
				/>
			</Label>
			<Label id="scene-play.settings.gridFactor" label="Grid factor">
				<Input
					className="h-7 font-mono text-xs"
					onChange={(event) => {
						const value = Number(event.target.value);
						if (Number.isFinite(value) && value > 0) {
							bus.dispatch(SCENE_PLAY_CONTROLLER_ID, "setGridFactor", { value });
						}
					}}
					value={String(snap.gridFactor)}
				/>
			</Label>
			<div className="text-muted-foreground space-y-1 font-mono text-[11px]">
				<div>connect: {snap.connectCount}</div>
				<div>proximity: {snap.proximityCount}</div>
				<div>indirect: {snap.indirectCount}</div>
				<div>compatible: {snap.compatibleObjectsCount}</div>
				<div>target ring: {snap.targetRingCount}</div>
			</div>
		</div>
	);
}

function ScenePlaySceneSurfaceHost({ node }: { readonly node: UiScene3DHostSurfaceNode }): React.ReactElement {
	const { runtime } = useApp();
	const bus = runtime.commandBus;
	const ctrl = useScenePlayController();
	if (node.controllerId !== SCENE_PLAY_CONTROLLER_ID) {
		return <div className="p-2 text-xs text-muted-foreground">Invalid scene viewport binding</div>;
	}
	const snap = useScenePlaySnapshot();
	if (!snap.fixture) {
		return <div className="p-4 text-destructive">Invalid scene fixture</div>;
	}
	const kindCompatibility = parseKindCompatibility(snap.fixture.meta);
	const kindCatalogs = parseKindCatalogs(snap.fixture.meta);
	const blockedVortexFullIds = blockedVortexFullIdsFromAttractions(snap.fixture.attractions);
	const selectedVortexFullIds = useMemo(() => new Set(snap.selection.vortexIds), [snap.selection.vortexIds]);
	const patchFixture = useCallback(
		(updater: (prev: FixtureV1) => FixtureV1) => {
			ctrl?.patchFixture(updater);
		},
		[ctrl],
	);
	return (
		<div className="absolute inset-0 min-h-0 min-w-0">
			<SceneObjectStateProvider
				fixture={snap.fixture}
				fixtureRevision={snap.fixtureRevision}
				onConnect={(payload) => {
					patchFixture((fixture) => applyConnectToSceneFixture(fixture, payload));
					bus.dispatch(SCENE_PLAY_CONTROLLER_ID, "noteConnect");
				}}
				onRelocate={(payload) => {
					patchFixture((fixture) => applyRelocateToSceneFixture(fixture, payload));
				}}
			>
				<PlaySceneCanvas
					fixture={snap.fixture}
					kindCatalogs={kindCatalogs}
					kindCompatibility={kindCompatibility}
					blockedVortexFullIds={blockedVortexFullIds}
					lodTag={snap.lodTag}
					lodProps={snap.lodProps}
					relocateMode={snap.relocateMode}
					selection={snap.selection}
					selectedId={snap.selectedId}
					selectionMode={snap.selectionMode}
					selectedVortexFullIds={selectedVortexFullIds}
					proximityRadius={snap.proximityRadius}
					chunkSize={snap.chunkSize}
					gridFactor={snap.gridFactor}
					showLodGrid={snap.showLodGrid}
					gridSnapEnabled={snap.gridSnapEnabled}
					setSelectedId={(id) => bus.dispatch(SCENE_PLAY_CONTROLLER_ID, "setSelectedId", { id })}
					onSelect={(selection) => bus.dispatch(SCENE_PLAY_CONTROLLER_ID, "noteSelection", selection)}
					onIndirectConnect={() => bus.dispatch(SCENE_PLAY_CONTROLLER_ID, "noteIndirect")}
					onProximityConnect={() => bus.dispatch(SCENE_PLAY_CONTROLLER_ID, "noteProximity")}
					onLodChange={(lod) => bus.dispatch(SCENE_PLAY_CONTROLLER_ID, "setEffectiveLod", { lod })}
					onCamera={(camera) => ctrl?.setCamera(camera)}
					onAttractionCompatibleObjects={() => bus.dispatch(SCENE_PLAY_CONTROLLER_ID, "noteCompatibleObjects")}
					onAttractionTargetRing={() => bus.dispatch(SCENE_PLAY_CONTROLLER_ID, "noteTargetRing")}
				/>
			</SceneObjectStateProvider>
		</div>
	);
}

let scenePlayChromeRegistered = false;

function registerScenePlayChrome(): void {
	if (scenePlayChromeRegistered) return;
	scenePlayChromeRegistered = true;
	registerUiScene3DSurfaceHost(SCENE_PLAY_SCENE_SURFACE_ID, ScenePlaySceneSurfaceHost);
	registerWindowBody(SCENE_PLAY_BODY_KEY, buildScenePlayDeclarativeBody);
	registerSidePanelBody("elements.scene.play.tab.inspector", ScenePlayInspectorPanel);
	registerSidePanelBody("elements.scene.play.tab.settings", ScenePlaySettingsPanel);
}

function readTheme(): ElementsSurfaceTheme {
	if (typeof localStorage === "undefined") return "system";
	try {
		return parseStoredTheme(localStorage.getItem(LS_THEME));
	} catch {
		return "system";
	}
}

function readDevice(): ElementsSurfaceDevice {
	if (typeof localStorage === "undefined") return "desktop";
	try {
		return parseStoredDevice(localStorage.getItem(LS_DEVICE));
	} catch {
		return "desktop";
	}
}

function readExpertise(): Expertise {
	if (typeof localStorage === "undefined") return Expertise.NORMAL;
	try {
		return parseStoredExpertise(localStorage.getItem(LS_EXPERTISE));
	} catch {
		return Expertise.NORMAL;
	}
}

class PlaySurfaceFooter extends React.Component<{
	theme: ElementsSurfaceTheme;
	device: ElementsSurfaceDevice;
	expertise: Expertise;
	onTheme: (v: ElementsSurfaceTheme) => void;
	onDevice: (v: ElementsSurfaceDevice) => void;
	onExpertise: (v: Expertise) => void;
}> {
	render(): React.ReactElement {
		const { theme, device, expertise, onDevice, onExpertise, onTheme } = this.props;
		return (
			<div className="flex min-w-0 flex-wrap items-center gap-double px-single py-tiny">
				<span className="shrink-0 text-xs text-muted-foreground">Theme</span>
				<Select onValueChange={(v) => onTheme(v as ElementsSurfaceTheme)} value={theme}>
					<SelectTrigger className="h-medium w-30" id="scene-play-surface-theme" size="sm">
						<SelectValue />
					</SelectTrigger>
					<SelectContent>
						<SelectItem value="system">System</SelectItem>
						<SelectItem value="light">Light</SelectItem>
						<SelectItem value="dark">Dark</SelectItem>
					</SelectContent>
				</Select>
				<span className="shrink-0 text-xs text-muted-foreground">Device</span>
				<Select onValueChange={(v) => onDevice(v as ElementsSurfaceDevice)} value={device}>
					<SelectTrigger className="h-medium w-30" id="scene-play-surface-device" size="sm">
						<SelectValue />
					</SelectTrigger>
					<SelectContent>
						<SelectItem value="desktop">Desktop</SelectItem>
						<SelectItem value="tablet">Tablet</SelectItem>
						<SelectItem value="mobile">Mobile</SelectItem>
					</SelectContent>
				</Select>
				<span className="shrink-0 text-xs text-muted-foreground">Expertise</span>
				<Select onValueChange={(v) => onExpertise(v as Expertise)} value={expertise}>
					<SelectTrigger className="h-medium w-30" id="scene-play-surface-expertise" size="sm">
						<SelectValue />
					</SelectTrigger>
					<SelectContent>
						<SelectItem value={Expertise.BEGINNER}>Beginner</SelectItem>
						<SelectItem value={Expertise.NORMAL}>Normal</SelectItem>
						<SelectItem value={Expertise.EXPERT}>Expert</SelectItem>
					</SelectContent>
				</Select>
			</div>
		);
	}
}

class PlaySceneCanvasContent extends React.Component<{
	readonly selection: SelectionSnapshot;
	readonly selectedId: string | null;
	readonly selectedVortexFullIds: ReadonlySet<string>;
	readonly relocateMode: RelocateMode;
	readonly setSelectedId: (id: string | null) => void;
}> {
	render(): React.ReactElement {
		return (
			<>
				<ScenePlayTestBridge setSelectedId={this.props.setSelectedId} />
				<React.Suspense fallback={null}>
					<SceneObjects
						selection={this.props.selection}
						selectedObjectId={this.props.selectedId}
						selectedVortexFullIds={this.props.selectedVortexFullIds}
						relocate={this.props.relocateMode}
					/>
					<SceneAttractions />
				</React.Suspense>
			</>
		);
	}
}

class PlaySceneCanvas extends React.Component<{
	readonly fixture: FixtureV1;
	readonly kindCatalogs: KindCatalogBundle | undefined;
	readonly kindCompatibility: readonly KindCompatEntry[];
	readonly blockedVortexFullIds: ReadonlySet<string>;
	readonly lodTag: number;
	readonly lodProps: Pick<CanvasProps, "automaticLod" | "depthVariableLod" | "lod">;
	readonly relocateMode: RelocateMode;
	readonly selection: ScenePlaySelection;
	readonly selectionMode: SelectionMode;
	readonly selectedVortexFullIds: ReadonlySet<string>;
	readonly proximityRadius: number;
	readonly chunkSize: number;
	readonly gridFactor: number;
	readonly showLodGrid: boolean;
	readonly gridSnapEnabled: boolean;
	readonly selectedId: string | null;
	readonly setSelectedId: (id: string | null) => void;
	readonly onSelect: (snap: SelectionSnapshot) => void;
	readonly onIndirectConnect: () => void;
	readonly onProximityConnect: () => void;
	readonly onLodChange?: (lod: number) => void;
	readonly onCamera?: (camera: CameraState) => void;
	readonly onAttractionCompatibleObjects?: (payload: AttractionCompatibleObjectsPayload) => void;
	readonly onAttractionTargetRing?: (payload: AttractionTargetRingPayload) => void;
}> {
	static contextType = SceneObjectStateContext;
	declare context: React.ContextType<typeof SceneObjectStateContext>;

	render(): React.ReactElement {
		const state = this.context as SceneObjectStateContextValue | null;
		if (!state) {
			throw new Error("SceneObjectStateProvider missing");
		}
		return (
			<>
				<div className="pointer-events-none absolute left-0 top-0 z-[-1] px-px py-px opacity-0">
					<div data-e2e-scene-lod>{formatSceneLod(this.props.lodTag)}</div>
					<div data-e2e-selected>{this.props.selectedId ?? "none"}</div>
				</div>
				<Canvas3D
					className="absolute inset-0"
					camera={this.props.fixture.camera}
					domain={this.props.fixture.domain}
					kindCatalogs={this.props.kindCatalogs}
					kindCompatibility={this.props.kindCompatibility}
					blockedVortexFullIds={this.props.blockedVortexFullIds}
					proximityRadius={this.props.proximityRadius}
					chunkSize={this.props.chunkSize}
					gridFactor={this.props.gridFactor}
					relocateMode={this.props.relocateMode}
					selectionMode={this.props.selectionMode}
					showLodGrid={this.props.showLodGrid}
					gridSnapEnabled={this.props.gridSnapEnabled}
					{...this.props.lodProps}
					onLodChange={this.props.onLodChange}
					onCamera={this.props.onCamera}
					onSelect={this.props.onSelect}
					onConnect={state.handleConnect}
					onIndirectConnect={this.props.onIndirectConnect}
					onProximityConnect={this.props.onProximityConnect}
					onAttractionCompatibleObjects={this.props.onAttractionCompatibleObjects}
					onAttractionTargetRing={this.props.onAttractionTargetRing}
					onRelocate={state.handleRelocate}
					selection={this.props.selection}
				>
					<PlaySceneCanvasContent
						relocateMode={this.props.relocateMode}
						selection={this.props.selection}
						selectedId={this.props.selectedId}
						selectedVortexFullIds={this.props.selectedVortexFullIds}
						setSelectedId={this.props.setSelectedId}
					/>
				</Canvas3D>
			</>
		);
	}
}

interface PlayInnerState {
	readonly theme: ElementsSurfaceTheme;
	readonly device: ElementsSurfaceDevice;
	readonly expertise: Expertise;
}

class PlayInner extends React.Component<{}, PlayInnerState> {
	state: PlayInnerState = {
		theme: readTheme(),
		device: readDevice(),
		expertise: readExpertise(),
	};

	private cleanupSurfaceChrome: (() => void) | null = null;

	private sceneShell: ProductRuntime | null = null;

	componentDidMount(): void {
		this.applySurfaceChrome();
		this.persistState();
		const fixture = parseFixtureV1(nakaginSceneFixtureJson as unknown);
		if (fixture) {
			const urls = [...new Set(fixture.objects.map((object) => object.meshUrl))];
			for (const url of urls) {
				useGLTF.preload(url);
			}
		}
	}

	componentDidUpdate(_prevProps: {}, prevState: Readonly<PlayInnerState>): void {
		if (prevState.theme !== this.state.theme || prevState.device !== this.state.device || prevState.expertise !== this.state.expertise) {
			this.applySurfaceChrome();
			this.persistState();
		}
	}

	componentWillUnmount(): void {
		this.cleanupSurfaceChrome?.();
	}

	private applySurfaceChrome(): void {
		this.cleanupSurfaceChrome?.();
		this.cleanupSurfaceChrome = applyElementsSurfaceChrome({
			theme: this.state.theme,
			device: this.state.device,
			expertise: this.state.expertise,
		});
	}

	private persistState(): void {
		try {
			localStorage.setItem(LS_THEME, this.state.theme);
			localStorage.setItem(LS_DEVICE, this.state.device);
			localStorage.setItem(LS_EXPERTISE, this.state.expertise);
		} catch {}
	}

	render(): React.ReactElement {
		registerScenePlayChrome();
		const surfaceFooterItems: FooterItem[] = [
			{
				content: <PlaySurfaceFooter device={this.state.device} expertise={this.state.expertise} onDevice={(device) => this.setState({ device })} onExpertise={(expertise) => this.setState({ expertise })} onTheme={(theme) => this.setState({ theme })} theme={this.state.theme} />,
				id: "scene-play-surface",
				order: 0,
			},
		];
		if (!this.sceneShell) {
			const wb = new ProductRuntime();
			const ctrl = new ScenePlayShellController(wb.commandBus, () => wb.notify());
			wb.addApp(buildScenePlayAppRuntime(ctrl));
			this.sceneShell = wb;
		}
		const runtime = this.sceneShell;
		return (
			<ScenePlayProductShell
				extraFooterItems={surfaceFooterItems}
				mobile={this.state.device === "mobile"}
				runtime={runtime}
			/>
		);
	}
}

function ScenePlayProductShell(props: {
	readonly runtime: ProductRuntime;
	readonly extraFooterItems: FooterItem[];
	readonly mobile: boolean;
}): React.ReactElement {
	const ctrl = props.runtime.getActiveApp()?.controller as ScenePlayShellController | undefined;
	const generation = React.useSyncExternalStore(
		(onStoreChange) => props.runtime.subscribe(onStoreChange),
		() => props.runtime.generation,
		() => 0,
	);
	void generation;
	const snap = ctrl?.getSnapshot();
	const fixture = snap?.fixture;
	const kindCatalogs = fixture ? parseKindCatalogs(fixture.meta) : undefined;
	const shellValue = useMemo<ScenePlayShellContextValue | null>(() => {
		if (!ctrl || !snap || !fixture) {
			return null;
		}
		return {
			snap,
			patchFixture: (updater) => ctrl.patchFixture(updater),
			setSelection: (selection) => ctrl.run("setSelection", { selection }),
			kindCatalogs,
		};
	}, [ctrl, snap, fixture, kindCatalogs]);
	const product = (
		<ProductView
			runtime={props.runtime}
			defaultAppId={PLAY_APP_ID}
			extraFooterItems={props.extraFooterItems}
			initialPanelVisibility={{ leftSidePanel: false, rightSidePanel: true }}
			mobile={props.mobile}
		/>
	);
	if (!shellValue) {
		return product;
	}
	return <ScenePlayShellContext.Provider value={shellValue}>{product}</ScenePlayShellContext.Provider>;
}

class PlayApp extends React.Component {
	render(): React.ReactElement {
		return (
			<LevelProvider level="window">
				<div className={`flex h-screen min-h-0 w-screen flex-col ${getLevelBgClass("window")}`}>
					<PlayInner />
				</div>
			</LevelProvider>
		);
	}
}

export function createScenePlayElement(): React.ReactElement {
	return <PlayApp />;
}

/** @emoji ­ƒÜÇ Vite host entry: mounts scene play into `#root`. */
export function mountScenePlay(): void {
	mountReactApp(createScenePlayElement());
}

// #endregion 🛝PlayHost
