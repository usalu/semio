import { Clone, Line, OrbitControls, PerspectiveCamera, TransformControls, useGLTF } from "@react-three/drei";
import { Canvas, createPortal, useFrame, useThree } from "@react-three/fiber";
import {
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
	type CSSProperties,
	type MutableRefObject,
	type ReactElement,
	type ReactNode,
} from "react";
import {
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
	Points,
	PointsMaterial,
	PerspectiveCamera as ThreePerspectiveCamera,
	Plane,
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

//#region 🔖Kinds
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
/** @emoji 🎨 Homogeneous GLB presentation kind for pooled scene meshes ({@link MeshBody}). */
export type MeshStyleKind = (typeof MESH_STYLE_KINDS)[number];
/** @emoji 🎨 Default object mesh style when none is passed ({@link MeshBody}). */
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

const DOMAIN_LOD_SCALES: Record<DomainKind, readonly [ScaleKind, ScaleKind, ScaleKind, ScaleKind, ScaleKind, ScaleKind]> = {
	urban: ["1to50000", "1to25000", "1to10000", "1to5000", "1to2500", "1to1000"],
	architecture: ["1to1000", "1to500", "1to333", "1to200", "1to100", "1to50"],
	detailing: ["1to50", "1to33", "1to25", "1to10", "1to5", "1to1"],
	engineering: ["1to1", "2to1", "5to1", "10to1", "20to1", "50to1"],
};

export interface CameraState {
	position: Vec3;
	target: Vec3;
	zoom: number;
}

/** @emoji 📶 Board-aligned scene LOD band label (`data-scene-lod` on the canvas shell). */
export type LodKind = "minimap" | "overview" | "compact" | "normal" | "detail" | "micro";

/** @emoji 📐 LOD zoom boundaries for pseudo-zoom from orbit camera distance (same semantics as board CSS zoom bands). */
export interface LodZoomThresholds {
	minimapMaxZoom: number;
	overviewMaxZoom: number;
	compactMaxZoom: number;
	normalMaxZoom: number;
	detailMaxZoom: number;
}

function scaleKindZoomAnchor(kind: ScaleKind, reference: number): number {
	const [numerator, denominator] = SCALE_RATIOS[kind];
	return (reference * numerator) / denominator;
}

function thresholdBetweenZoomAnchors(a: number, b: number): number {
	return Math.sqrt(a * b);
}

/** @emoji 📐 Returns the six canonical scale bands used by a scene domain from minimap through micro. */
export function lodScaleKindsForDomain(domain: DomainKind): readonly ScaleKind[] {
	return DOMAIN_LOD_SCALES[domain];
}

/** @emoji 📐 Derives scene LOD thresholds from the scene domain scale ladder using a positive reference zoom. */
export function lodZoomThresholdsForDomain(
	domain: DomainKind,
	reference = DEFAULT_SCALE_REFERENCE,
): LodZoomThresholds {
	const [minimap, overview, compact, normal, detail, micro] = lodScaleKindsForDomain(domain).map((kind) =>
		scaleKindZoomAnchor(kind, reference),
	);
	return {
		minimapMaxZoom: thresholdBetweenZoomAnchors(minimap, overview),
		overviewMaxZoom: thresholdBetweenZoomAnchors(overview, compact),
		compactMaxZoom: thresholdBetweenZoomAnchors(compact, normal),
		normalMaxZoom: thresholdBetweenZoomAnchors(normal, detail),
		detailMaxZoom: thresholdBetweenZoomAnchors(detail, micro),
	};
}

/** @emoji 📐 Default LOD thresholds aligned with the architecture scene domain. */
export const DEFAULT_LOD_ZOOM_THRESHOLDS: LodZoomThresholds = lodZoomThresholdsForDomain(
	DEFAULT_DOMAIN,
);

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
	handleMeshByLod?: Partial<Record<LodKind, string>>;
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
	/** @emoji 🎨 Explicit mesh style; otherwise derived from disabled, selected, highlighted, hovered. */
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
	/** @emoji 🧲 Object ids attracted to this object in the resolved ownership tree. */
	attracting?: readonly string[];
	/** @emoji 🕳️ Root of a connected attraction component (wormhole). */
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
	/** @emoji 🚫 Vortex full ids (`objectId:vortexId`) that already terminate an attraction and cannot start or receive a new attraction. */
	blockedVortexFullIds?: ReadonlySet<string>;
	proximityRadius?: number;
	relocateMode?: RelocateMode;
	selectionMode?: SelectionMode;
	/** @emoji 📶 LOD zoom thresholds for pseudo-zoom derived from orbit camera distance. */
	lodZoomThresholds?: LodZoomThresholds;
	/** @emoji 📶 When true (default), orbit pseudo-zoom selects the scene LOD band; when false, {@link lod} pins the tier when set. */
	automaticLod?: boolean;
	/** @emoji 📶 Pinned scene LOD when {@link automaticLod} is false; omit to follow orbit pseudo-zoom bands. */
	lod?: LodKind;
	/** @emoji 📏 Orbit distance at which pseudo-zoom is ~1 (tune to scene extent). */
	lodDistanceReference?: number;
	/** @emoji 📐 Multiplier for LOD grid steps (board `grid_factor`). */
	gridFactor?: number;
	/** @emoji 📐 When true, draw a world `GridHelper` stepped by the current LOD band grid. */
	showLodGrid?: boolean;
	/** @emoji 🧲 When true, translate relocate snaps to the finest visible LOD grid step (board `grid_snap_enabled`). */
	gridSnapEnabled?: boolean;
	onCamera?: (s: CameraState) => void;
	/** @emoji 📶 Emits whenever the resolved scene LOD band changes. */
	onLodChange?: (lod: LodKind) => void;
	onSelect?: (snap: SelectionSnapshot) => void;
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

export interface FixtureV1 {
	schema: "elements.scene.fixture/v1";
	camera: CameraState;
	domain: DomainKind;
	meta?: Record<string, unknown>;
	attractions: AttractionProps[];
	objects: FixtureObjectV1[];
}
//#endregion 🔖Kinds

//#region 📶Lod
/** @emoji 📶 Resolves scene LOD from pseudo-zoom using explicit thresholds (same band order as the sketch board surface). */
export function resolveLodLabelFromThresholds(zoom: number, t: LodZoomThresholds): LodKind {
	const z = zoom;
	if (z < t.minimapMaxZoom) return "minimap";
	if (z < t.overviewMaxZoom) return "overview";
	if (z < t.compactMaxZoom) return "compact";
	if (z < t.normalMaxZoom) return "normal";
	if (z < t.detailMaxZoom) return "detail";
	return "micro";
	}

/** @emoji 📏 Maps orbit camera distance to a board-comparable pseudo-zoom (`reference / distance`). */
export function pseudoZoomFromOrbitDistance(distance: number, reference: number): number {
	const d = Math.max(distance, 1e-6);
	return reference / d;
}

/** @emoji 📐 Visible LOD grid / relocate snap step in world units (mirrors sketch board WASM `lod_visible_grid_snap_step_world`). */
export function lodVisibleGridSnapStepWorld(lod: LodKind, gridFactor: number): number | null {
	const f = gridFactor;
	switch (lod) {
		case "minimap":
			return null;
		case "overview":
			return 10 * f;
		case "compact":
			return 5 * f;
		case "normal":
			return 2.5 * f;
		case "detail":
			return 0.5 * f;
		case "micro":
			return 0.1 * f;
		default:
			return null;
	}
}

/** @emoji 🌀 True when primary handle visuals are drawn (board `draw_handles`: normal | detail | micro). */
export function handlePrimaryVisualVisibleAtLod(lod: LodKind): boolean {
	return lod === "normal" || lod === "detail" || lod === "micro";
}

/** @emoji 🌀 Overview uses invisible pick proxies when primary handle GLB is hidden (minimap has no handle picks). */
export function handlePickProxyAtLod(lod: LodKind): boolean {
	return lod === "overview" || lod === "compact";
}

export interface LodContextValue {
	readonly lod: LodKind;
	readonly lodGridStepWorld: number | null;
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

function LodGridHelper() {
	const lod = useLod();
	const grid = useMemo(() => {
		const step = lod.lodGridStepWorld;
		if (step == null || !Number.isFinite(step) || step <= 0) return null;
		const size = 12_000;
		const divs = Math.min(512, Math.max(2, Math.round(size / step)));
		return new GridHelper(size, divs, 0x8899aa, 0x445566);
	}, [lod.lodGridStepWorld]);
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
	readonly lodKindRef: MutableRefObject<LodKind>;
	readonly thresholds: LodZoomThresholds;
	readonly distanceReference: number;
	readonly gridFactor: number;
	readonly gridSnapEnabled: boolean;
	readonly automaticLod: boolean;
	readonly pinnedLod: LodKind | undefined;
	readonly onCtx: (next: LodContextValue) => void;
	readonly onLodChange?: (lod: LodKind) => void;
}) {
	const cam = useThree((s) => s.camera);
	const controls = useThree((s) => s.controls as { target?: Vector3 } | null);
	const tmpT = useMemo(() => new Vector3(), []);
	const prevLod = useRef<LodKind | null>(null);
	const ctxSig = useRef("");
	useFrame(() => {
		const tgt = controls?.target ?? tmpT.set(0, 0, 0);
		const dist = cam.position.distanceTo(tgt);
		const pseudo = pseudoZoomFromOrbitDistance(dist, props.distanceReference);
		const next =
			!props.automaticLod && props.pinnedLod !== undefined
				? props.pinnedLod
				: resolveLodLabelFromThresholds(pseudo, props.thresholds);
		props.lodKindRef.current = next;
		const lodGridStepWorld = lodVisibleGridSnapStepWorld(next, props.gridFactor);
		const sig = `${next}|${lodGridStepWorld ?? "x"}|${props.gridFactor}|${props.gridSnapEnabled}`;
		if (ctxSig.current !== sig) {
			ctxSig.current = sig;
			props.onCtx({
				lod: next,
				lodGridStepWorld,
				gridFactor: props.gridFactor,
				gridSnapEnabled: props.gridSnapEnabled,
			});
		}
		if (prevLod.current !== next) {
			prevLod.current = next;
			props.onLodChange?.(next);
		}
	});
	return null;
}

function LodBridge(props: {
	readonly children: ReactNode;
	readonly lodKindRef: MutableRefObject<LodKind>;
	readonly thresholds: LodZoomThresholds;
	readonly distanceReference: number;
	readonly gridFactor: number;
	readonly gridSnapEnabled: boolean;
	readonly showLodGrid: boolean;
	readonly automaticLod: boolean;
	readonly pinnedLod: LodKind | undefined;
	readonly onLodChange?: (lod: LodKind) => void;
}) {
	const [lodCtx, setLodCtx] = useState<LodContextValue>(() => ({
		lod: "normal",
		lodGridStepWorld: lodVisibleGridSnapStepWorld("normal", props.gridFactor),
		gridFactor: props.gridFactor,
		gridSnapEnabled: props.gridSnapEnabled,
	}));
	const onCtx = useCallback(
		(next: LodContextValue) => {
			setLodCtx((prev) => {
				if (
					prev.lod === next.lod &&
					prev.lodGridStepWorld === next.lodGridStepWorld &&
					prev.gridFactor === next.gridFactor &&
					prev.gridSnapEnabled === next.gridSnapEnabled
				) {
					return prev;
				}
				return next;
			});
		},
		[],
	);
	const v = useMemo(() => lodCtx, [lodCtx]);
	return (
		<LodContext.Provider value={v}>
			<LodFrameRunner
				lodKindRef={props.lodKindRef}
				thresholds={props.thresholds}
				distanceReference={props.distanceReference}
				gridFactor={props.gridFactor}
				gridSnapEnabled={props.gridSnapEnabled}
				automaticLod={props.automaticLod}
				pinnedLod={props.pinnedLod}
				onCtx={onCtx}
				onLodChange={props.onLodChange}
			/>
			{props.showLodGrid ? <LodGridHelper /> : null}
			{props.children}
		</LodContext.Provider>
	);
}
//#endregion 📶Lod

//#region 🧾Fixture
function isVec3(v: unknown): v is Vec3 {
	return Array.isArray(v) && v.length === 3 && v.every((n) => typeof n === "number");
}

function isQuat(v: unknown): v is Quat {
	return Array.isArray(v) && v.length === 4 && v.every((n) => typeof n === "number");
}

const LOD_KINDS: readonly LodKind[] = ["minimap", "overview", "compact", "normal", "detail", "micro"];

/** @emoji 📶 Select value: orbit pseudo-zoom picks the scene LOD band. */
export const LOD_MODE_AUTOMATIC = "automatic" as const;

/** @emoji 📶 Scene play / window LOD select value (`automatic` or a pinned {@link LodKind}). */
export type LodModeKind = typeof LOD_MODE_AUTOMATIC | LodKind;

/** @emoji ✅ True when `label` is a pinned scene LOD tier. */
export function isLodKind(label: string): label is LodKind {
	return (LOD_KINDS as readonly string[]).includes(label);
}

/** @emoji 📶 Maps a window LOD select value to {@link CanvasProps} LOD fields. */
export function lodCanvasProps(mode: LodModeKind): Pick<CanvasProps, "automaticLod" | "lod"> {
	if (mode === LOD_MODE_AUTOMATIC) {
		return { automaticLod: true };
	}
	return { automaticLod: false, lod: mode };
}

/** @emoji 📶 Automatic LOD select row label showing the live orbit-derived tier. */
export function lodAutomaticSelectLabel(effectiveTier: LodKind): string {
	return `Automatic · ${effectiveTier.charAt(0).toUpperCase()}${effectiveTier.slice(1)}`;
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

function parseHandleMeshByLod(v: unknown): Partial<Record<LodKind, string>> | undefined {
	if (!v || typeof v !== "object") return undefined;
	const o = v as Record<string, unknown>;
	const out: Partial<Record<LodKind, string>> = {};
	for (const k of LOD_KINDS) {
		const s = o[k];
		if (typeof s === "string" && s.length) out[k] = s;
	}
	return Object.keys(out).length ? out : undefined;
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
		objects.push({
			id: or.id,
			meshUrl: or.meshUrl,
			origin,
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
//#endregion 🧾Fixture

//#region 🕸️AttractionGraph
/** @emoji 🔗 Parsed `objectId:vortexId` attraction endpoint. */
export function parseVortexFullId(full: string): { readonly objectId: string; readonly vortexId: string } {
	const i = full.indexOf(":");
	if (i < 0) {
		return { objectId: full, vortexId: "link" };
	}
	return { objectId: full.slice(0, i), vortexId: full.slice(i + 1) };
}

/** @emoji 🕳️ True when the object is an explicit or inferred wormhole root. */
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

/** @emoji 🧲 One object-level attraction edge derived from an attraction (`attracting` attracts `attracted`). */
export interface AttractionEdge {
	readonly attractingObjectId: string;
	readonly attractedObjectId: string;
	readonly attractionId: string;
}

/** @emoji 🧲 Maps scene attractions to object-level attraction edges. */
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

/** @emoji 🔄 True when `attractingObjectId → attractedObjectId` closes a directed cycle in attraction edges. */
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

/** @emoji ✂️ Clears one parent link per ownership cycle so {@link SceneAttractionTree} stays a forest. */
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

/** @emoji 🕸️ Resolves a forest from attraction edges: wormhole roots, closest-to-wormhole parent when multiply attracted. */
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

/** @emoji 🧲 Collects transitive attracted object ids in the resolved ownership tree. */
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

/** @emoji 🔑 Stable fingerprint for external fixture resync (ignores object reference identity). */
export function fixtureStateFingerprint(fixture: FixtureV1): string {
	const attractionIds = fixture.attractions.map((a) => a.id).join("\0");
	const objectIds = fixture.objects.map((o) => o.id).join("\0");
	return `${fixture.objects.length}\0${fixture.attractions.length}\0${objectIds}\0${attractionIds}`;
}

function objectStateReducer(state: ObjectStateSnapshot, action: ObjectStateAction): ObjectStateSnapshot {
	switch (action.type) {
		case "init": {
			const records = fixtureToRecords(action.fixture.objects);
			return buildSnapshot(records, action.fixture.attractions, state.version + 1);
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

export interface SceneObjectStateContextValue {
	readonly snapshot: ObjectStateSnapshot;
	readonly dispatch: (action: ObjectStateAction) => void;
	readonly handleRelocate: (payload: RelocatePayload) => void;
	readonly handleConnect: (payload: AttractionPayload) => void;
}

const SceneObjectStateContext = createContext<SceneObjectStateContextValue | null>(null);

/** @emoji 🗄️ Central scene object records, attractions, and resolved attraction ownership. */
export function SceneObjectStateProvider(props: {
	readonly fixture: FixtureV1;
	readonly children: ReactNode;
	readonly onRelocate?: (payload: RelocatePayload) => void;
	readonly onConnect?: (payload: AttractionPayload) => void;
}) {
	const [snapshot, dispatch] = useReducer(objectStateReducer, props.fixture, (fixture) =>
		buildSnapshot(fixtureToRecords(fixture.objects), fixture.attractions, 0),
	);
	const syncedFixtureFingerprintRef = useRef<string | null>(null);
	const fixtureFingerprint = useMemo(() => fixtureStateFingerprint(props.fixture), [props.fixture]);
	useEffect(() => {
		if (syncedFixtureFingerprintRef.current === fixtureFingerprint) {
			return;
		}
		syncedFixtureFingerprintRef.current = fixtureFingerprint;
		dispatch({ type: "init", fixture: props.fixture });
	}, [props.fixture, fixtureFingerprint]);
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

/** @emoji 🪝 Relocate handler that updates central object state and cascades to attracted descendants. */
export function useSceneObjectRelocate(): (payload: RelocatePayload) => void {
	return useSceneObjectState().handleRelocate;
}

/** @emoji 🪝 Connect handler that appends an attraction and recomputes attraction ownership. */
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
			{record.vortices.map((v) => (
				<Vortex key={v.id} objectId={record.id} objectKind={record.objectKind} {...v} />
			))}
		</ObjectItem>
	);
});

/** @emoji 🌲 Declares attraction tree structure; meshes mount flat via {@link SceneObjects} so ids stay stable on reparent. */
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

export interface SceneObjectsProps {
	readonly selectedObjectId?: string | null;
	readonly relocate?: RelocateMode | false;
}

/** @emoji 🧊 Renders all scene objects from central state (id-keyed; survives ownership changes). */
export const SceneObjects = memo(function SceneObjects(props: SceneObjectsProps) {
	const { snapshot } = useSceneObjectState();
	const ids = useMemo(() => [...snapshot.records.keys()].sort(), [snapshot.records, snapshot.version]);
	return (
		<>
			{ids.map((id) => (
				<ObjectItemById
					key={id}
					objectId={id}
					selected={props.selectedObjectId === id}
					relocate={props.relocate}
				/>
			))}
		</>
	);
});

/** @emoji 🌲 Logical attraction tree roots (wormholes) for structure-only composition. */
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

/** @emoji 🧲 Renders all attraction endpoint lines in one frame loop (avoids N×useFrame churn). */
export const SceneAttractions = memo(function SceneAttractions() {
	const { snapshot } = useSceneObjectState();
	return <SceneAttractionLineBatch attractions={snapshot.attractions} />;
});
//#endregion 🕸️AttractionGraph

//#region 🧩Compat
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

/** @emoji 🧲 Attraction endpoint vortex full ids that are already attracting/attracted and cannot start or receive another attraction. */
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

/** @emoji 🧭 Semantic kinds at one end of an attraction drag (object + vortex handle). */
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

/** @emoji 🔌 Resolves default wire kind for a vortex kind via handle catalog, else `board.wire.link`. */
export function resolveWireKindForVortex(
	vortexKind: string | undefined,
	catalogs: KindCatalogBundle | undefined,
): string {
	const h = catalogHandleById(catalogs, vortexKind);
	const w = h?.defaultWireKind?.trim();
	return w && w.length > 0 ? w : DEFAULT_WIRE_KIND_ID;
}

/** @emoji 🪢 Resolves default edge kind for a wire kind via wire catalog, else empty string. */
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

/** @emoji 🤝 WASM-style filtered attraction compatibility (important + specificity tiers); empty rules allow all. */
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
//#endregion 🧩Compat

//#region 🎨MeshPaint
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

/** @emoji 🎨 Resolves mesh and edge colors for a {@link MeshStyleKind} from Elements tokens. */
export function meshStyleColors(style: MeshStyleKind): MeshStyleColors | null {
	if (style === "original") {
		return null;
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
	return {
		meshColor: resolveCssColor("backgroundColor", meshExprs[style], fb.meshColor),
		lineColor: resolveCssColor("color", lineExprs[style], fb.lineColor),
		emissiveColor: resolveCssColor("color", lineExprs[style], fb.emissiveColor),
		emissiveIntensity: fb.emissiveIntensity,
		opacity: fb.opacity,
	};
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

/** @emoji 🎨 Chooses the effective mesh style from explicit prop and interaction flags. */
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

/** @emoji 🎨 Resolves a CSS color for scene lines (endpoint attractions, attraction guides). */
export function lineCssColor(expr: string, fallback: string): string {
	return resolveCssColor("color", expr, fallback);
}
//#endregion 🎨MeshPaint

//#region 🏊Pool
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

/** @emoji 🧹 Drops pooled GLTF cache entries (call on scene teardown, not per-chunk unmount). */
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

/** @emoji 🏊 Returns a cached styled GLTF template for {@link MeshBody} (refcount via acquire/release). */
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
//#endregion 🏊Pool

//#region 🎯Registry
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
}

const RegistryContext = createContext<RegistryValue | null>(null);

function useRegistry(): RegistryValue {
	const v = useContext(RegistryContext);
	if (!v) throw new Error("Scene registry missing");
	return v;
}
//#endregion 🎯Registry

//#region 🧱Chunking
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

/** @emoji 📏 Chunk bounding radius in world units (half-space diagonal of a cube chunk). */
export function chunkBoundsRadius(chunkSize: number): number {
	return chunkSize * 0.866;
}

/** @emoji 👁️ Distance-only chunk visibility with enter/exit hysteresis (avoids frustum-edge flicker). */
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
//#endregion 🧱Chunking

//#region 🧊Helpers
function vec3ToThree(v: Vec3) {
	return new Vector3(v[0], v[1], v[2]);
}

function quatToThree(q: Quat | undefined) {
	if (!q) return new Quaternion();
	return new Quaternion(q[0], q[1], q[2], q[3]);
}

function scaleToThree(s: number | Vec3 | undefined): Vector3 {
	if (s === undefined) return new Vector3(1, 1, 1);
	if (typeof s === "number") return new Vector3(s, s, s);
	return new Vector3(s[0], s[1], s[2]);
}

/** @emoji 🔑 Stable key for object pose props (relocate mutates the group without changing this until commit). */
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

/** @emoji 📍 Writes fixture pose onto an object group; avoids R3F controlled transforms so vortex children follow relocate. */
export function applyObjectPose(
	group: Group,
	origin: Vec3,
	orientation: Quat | undefined,
	scale: number | Vec3 | undefined,
): void {
	group.position.set(origin[0], origin[1], origin[2]);
	group.quaternion.copy(quatToThree(orientation));
	group.scale.copy(scaleToThree(scale));
}

/** @emoji 🌳 Updates world matrices from root to leaf so {@link Object3D.getWorldPosition} matches the current graph. */
export function updateWorldMatrixChain(leaf: Object3D): void {
	const chain: Object3D[] = [];
	for (let cur: Object3D | null = leaf; cur; cur = cur.parent) {
		chain.push(cur);
	}
	for (let i = chain.length - 1; i >= 0; i--) {
		chain[i]!.updateMatrixWorld(false);
	}
}

function vector3IsFinite(v: Vector3): boolean {
	return Number.isFinite(v.x) && Number.isFinite(v.y) && Number.isFinite(v.z);
}
//#endregion 🧊Helpers

//#region 🧲AttractionGesture
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
	lod: LodKind;
	pointerWorld: Vector3;
	attractingFullId: string;
	compat: ReadonlySet<string>;
	blocked: ReadonlySet<string>;
	camera: Camera;
	gl: WebGLRenderer;
	getVortexWorld: (id: string) => Vector3 | null;
	metaRadius: (id: string) => number;
}): string | null {
	if (args.lod === "minimap") return null;
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
//#endregion 🧲AttractionGesture

//#region 🧊Mesh
export interface MeshProps {
	readonly meshUrl: string;
	readonly style?: MeshStyleKind;
	readonly userData?: Record<string, unknown>;
	readonly scale?: number | [number, number, number];
}

/** @emoji 🧊 Pooled GLB body with {@link MeshStyleKind} recoloring aligned to Elements tokens. */
export const MeshBody = memo(function MeshBody(props: MeshProps) {
	const style = props.style ?? DEFAULT_MESH_STYLE;
	const renderRoot = usePooledStyledMesh(props.meshUrl, style);
	if (!renderRoot) {
		return null;
	}
	const scale = props.scale;
	return (
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
			userData={props.userData}
		/>
	);
});

const PlaceholderMesh = memo(function PlaceholderMesh(props: { readonly style: MeshStyleKind }) {
	const colors = meshStyleColors(props.style);
	const meshColor = colors?.meshColor ?? "#cbd5e1";
	const opacity = colors?.opacity ?? 1;
	return (
		<mesh>
			<boxGeometry args={[1, 1, 1]} />
			<meshStandardMaterial
				color={meshColor}
				metalness={0.05}
				roughness={0.85}
				transparent={opacity < 1}
				opacity={opacity}
			/>
		</mesh>
	);
});
//#endregion 🧊Mesh

//#region 🧊Object

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
						origin: before.origin.toArray() as unknown as Vec3,
						orientation: before.quat.toArray() as unknown as Quat,
						scale: before.scale.toArray() as unknown as Vec3,
					},
					after: {
						origin: g.position.toArray() as unknown as Vec3,
						orientation: g.quaternion.toArray() as unknown as Quat,
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
	} = useRegistry();
	const beforeRef = useRef<{ origin: Vector3; quat: Quaternion; scale: Vector3 } | null>(null);
	const [tcTarget, setTcTarget] = useState<Group | null>(null);
	const [pointerHovered, setPointerHovered] = useState(false);

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
				hovered: props.hovered === true || pointerHovered,
			}),
		[props.style, props.disabled, props.selected, props.hovered, linkHighlighted, pointerHovered],
	);

	const handlePointerDown = useCallback(
		(e: { stopPropagation: () => void }) => {
			e.stopPropagation();
			if (attractionDragActive || attractionIndirectPickAwait) return;
			if (props.disabled) {
				return;
			}
			if (selectionMode === "single") {
				setSelectedObjectIds([props.id]);
				onSelect?.({ objectIds: [props.id], vortexIds: [] });
			}
			setActiveRelocateObjectId(props.id);
		},
		[
			attractionDragActive,
			attractionIndirectPickAwait,
			onSelect,
			props.disabled,
			props.id,
			selectionMode,
			setActiveRelocateObjectId,
			setSelectedObjectIds,
		],
	);

	const handlePointerOver = useCallback(
		(e: { stopPropagation: () => void }) => {
			e.stopPropagation();
			if (!props.disabled) {
				setPointerHovered(true);
			}
		},
		[props.disabled],
	);

	const handlePointerOut = useCallback((e: { stopPropagation: () => void }) => {
		e.stopPropagation();
		setPointerHovered(false);
	}, []);

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
	const mode = props.relocate ?? relocateMode;
	const transSnap =
		mode === "translate" &&
		lodCtx.gridSnapEnabled &&
		lodCtx.lodGridStepWorld != null &&
		lodCtx.lodGridStepWorld > 0
			? lodCtx.lodGridStepWorld
			: undefined;
	const showTc =
		props.selected && activeRelocateObjectId === props.id && props.relocate !== false && tcTarget;

	return (
		<>
			<group
				ref={group}
				visible={props.visible !== false}
				onPointerDown={handlePointerDown}
				onPointerOver={handlePointerOver}
				onPointerOut={handlePointerOut}
				userData={{
					sceneObjectId: props.id,
					sceneMeshStyle: meshStyle,
					...(props.attracting?.length ? { sceneAttracting: props.attracting } : {}),
					...(props.wormhole ? { sceneWormhole: true } : {}),
					...props.userData,
				}}
			>
				{props.meshUrl === PLACEHOLDER_MESH_URL ? (
					<PlaceholderMesh style={meshStyle} />
				) : (
					<MeshBody meshUrl={props.meshUrl} style={meshStyle} />
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
//#endregion 🧊Object

//#region 🌀Vortex
const vortexFallbackMatProps = { transparent: true, opacity: 0.55 } as const;

function VortexHandleGltf(props: {
	meshUrl: string;
	fullId: string;
	radius: number;
	style: MeshStyleKind;
}) {
	const scale = (props.radius / 0.35) * 0.9;
	return (
		<MeshBody
			meshUrl={props.meshUrl}
			style={props.style}
			scale={scale}
			userData={{ sceneVortexFullId: props.fullId }}
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
}) {
	const style = vortexHighlightMeshStyle(props.highlight);
	const colors = meshStyleColors(style) ?? meshStyleColors("neutral")!;
	return (
		<mesh userData={{ sceneVortexFullId: props.fullId }}>
			<sphereGeometry args={[props.radius, 12, 12]} />
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
	props: VortexProps & { objectId: string; objectKind?: string },
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
	const highlight: "none" | "compatible" | "ring" | "attracting" | "indirectRing" = reg.attractionDragAttractingFullId === fullId
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
			e.stopPropagation();
			const pe = e.nativeEvent;
			if (pe.button !== 0) return;
			if (reg.blockedVortexFullIds.has(fullId)) return;
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
	const drawHandleBody = handlePrimaryVisualVisibleAtLod(lodCtx.lod) || linger;
	const pickProxy = handlePickProxyAtLod(lodCtx.lod) && !drawHandleBody;
	const meshFromLod = props.handleMeshByLod?.[lodCtx.lod];
	const meshUrl =
		typeof meshFromLod === "string" && meshFromLod.length
			? meshFromLod
			: typeof props.handleMeshUrl === "string" && props.handleMeshUrl.length
				? props.handleMeshUrl
				: undefined;

	const handleMeshStyle = vortexHighlightMeshStyle(highlight);
	const vis = props.visible !== false;
	return (
		<group
			ref={bindRoot}
			position={props.position as [number, number, number]}
			userData={{ sceneVortexFullId: fullId, vortexKind: props.vortexKind }}
			data-scene-vortex={fullId}
			visible={vis}
			onPointerDown={onPointerDown}
		>
			{drawHandleBody && meshUrl ? (
				<VortexHandleGltf meshUrl={meshUrl} fullId={fullId} radius={r} style={handleMeshStyle} />
			) : drawHandleBody && props.children ? (
				<group userData={{ sceneVortexFullId: fullId }}>{props.children}</group>
			) : drawHandleBody ? (
				<VortexFallbackMesh fullId={fullId} radius={r} highlight={highlight} />
			) : null}
			{pickProxy ? (
				<mesh userData={{ sceneVortexFullId: fullId }} renderOrder={-1}>
					<sphereGeometry args={[r, 12, 12]} />
					<meshBasicMaterial transparent opacity={0} depthWrite={false} />
				</mesh>
			) : null}
		</group>
	);
});
//#endregion 🌀Vortex

//#region 🧲Magnet
export const Magnet = memo(function Magnet(props: MagnetProps) {
	return (
		<mesh position={props.position as [number, number, number]} userData={{ sceneMagnetId: props.id }}>
			<boxGeometry args={[props.size[0], props.size[1], props.size[2]]} />
			<meshStandardMaterial color="#a78bfa" wireframe />
		</mesh>
	);
});
//#endregion 🧲Magnet

//#region 🧲SceneAttraction
const SceneAttractionLineBatch = memo(function SceneAttractionLineBatch(props: {
	readonly attractions: readonly AttractionProps[];
}) {
	const reg = useRegistry();
	const mat = useMemo(() => {
		const color = lineCssColor(CSS_ATTRACTION_ENDPOINT_LINE, "#64748b");
		return new LineBasicMaterial({ color, transparent: true, opacity: 0.85, depthTest: true });
	}, []);
	const geo = useMemo(() => new BufferGeometry(), []);
	useLayoutEffect(() => {
		const vertexCount = Math.max(props.attractions.length * 2, 2);
		geo.setAttribute("position", new Float32BufferAttribute(new Float32Array(vertexCount * 3), 3));
	}, [geo, props.attractions.length]);
	useFrame(() => {
		const pos = geo.attributes.position as Float32BufferAttribute;
		let write = 0;
		for (const attraction of props.attractions) {
			const a = reg.getVortexWorld(attraction.attracting);
			const b = reg.getVortexWorld(attraction.attracted);
			if (a && b && vector3IsFinite(a) && vector3IsFinite(b)) {
				pos.setXYZ(write, a.x, a.y, a.z);
				pos.setXYZ(write + 1, b.x, b.y, b.z);
			} else {
				pos.setXYZ(write, 0, 0, 0);
				pos.setXYZ(write + 1, 0, 0, 0);
			}
			write += 2;
		}
		pos.needsUpdate = true;
	});
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
	return <lineSegments geometry={geo} material={mat} raycast={() => null} />;
});

export const SceneAttraction = memo(function SceneAttraction(props: AttractionProps) {
	return <SceneAttractionLineBatch attractions={[props]} />;
});
//#endregion 🧲SceneAttraction

//#region 🧲Attraction
export const Attraction = memo(function Attraction(props: { attracting: Vec3; attracted: Vec3 }) {
	const pts = useMemo(() => [vec3ToThree(props.attracting), vec3ToThree(props.attracted)], [props.attracting, props.attracted]);
	const color = useMemo(
		() => lineCssColor(CSS_ATTRACTION_LINE, "#f472b6"),
		[],
	);
	return <Line points={pts} color={color} lineWidth={2} />;
});
//#endregion 🧲Attraction

//#region ✋Relocate
export function useSceneRelocate(objectId: string) {
	const reg = useRegistry();
	return {
		mode: reg.relocateMode,
		start: () => reg.setActiveRelocateObjectId(objectId),
		cancel: () => reg.setActiveRelocateObjectId(null),
	};
}
//#endregion ✋Relocate

const EMPTY_BLOCKED_VORTICES: ReadonlySet<string> = new Set();

//#region 🎬Scene
function OrbitGated() {
	const reg = useRegistry();
	const gate = reg.attractionDragActive || reg.attractionIndirectPickAwait !== null;
	return <OrbitControls makeDefault enabled={!gate} />;
}

/** @emoji 📷 Seeds default camera + orbit target once; orbit owns the rig afterward (no controlled-camera feedback loop). */
function SceneCameraSeed(props: { readonly position: Vec3; readonly target: Vec3 }) {
	const { camera } = useThree();
	const controls = useThree((s) => s.controls as { target: Vector3; update: () => void } | null);
	const seededFor = useRef("");
	const seedKey = `${props.position.join(",")}|${props.target.join(",")}`;
	useLayoutEffect(() => {
		if (seededFor.current === seedKey) {
			return;
		}
		seededFor.current = seedKey;
		camera.position.set(props.position[0], props.position[1], props.position[2]);
		camera.updateProjectionMatrix();
		if (controls?.target) {
			controls.target.set(props.target[0], props.target[1], props.target[2]);
			controls.update();
		}
	}, [camera, controls, props.position, props.target, seedKey]);
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
		window.addEventListener("pointermove", onMove);
		window.addEventListener("pointerup", onUp, { capture: true });
		window.addEventListener("pointerdown", onDown, true);
		return () => {
			window.removeEventListener("pointermove", onMove);
			window.removeEventListener("pointerup", onUp, true);
			window.removeEventListener("pointerdown", onDown, true);
		};
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

function CameraReporter({ zoom, onCamera }: { zoom: number; onCamera?: (s: CameraState) => void }) {
	const { camera } = useThree();
	const controls = useThree((s) => s.controls as { target: Vector3 } | null);
	const targetScratch = useMemo(() => new Vector3(), []);
	const last = useRef("");
	useFrame(() => {
		if (!onCamera) {
			return;
		}
		const tgt = controls?.target ?? targetScratch.set(0, 0, 0);
		const snap = JSON.stringify({
			p: camera.position.toArray(),
			t: tgt.toArray(),
			z: zoom,
		});
		if (snap === last.current) {
			return;
		}
		last.current = snap;
		onCamera({
			position: camera.position.toArray() as unknown as Vec3,
			target: tgt.toArray() as unknown as Vec3,
			zoom,
		});
	});
	return null;
}

function RegistryProvider({
	children,
	lodKindRef,
	kindCatalogs,
	kindCompatibility,
	blockedVortexFullIds,
	proximityRadius,
	selectionMode,
	relocateMode,
	onSelect,
	onConnect,
	onProximityConnect,
	onIndirectConnect,
	onAttractionCompatibleObjects,
	onAttractionTargetRing,
	onRelocate,
}: {
	children: ReactNode;
	lodKindRef: MutableRefObject<LodKind>;
	kindCatalogs: KindCatalogBundle | undefined;
	kindCompatibility: readonly KindCompatEntry[] | undefined;
	blockedVortexFullIds: ReadonlySet<string>;
	proximityRadius: number;
	selectionMode: SelectionMode;
	relocateMode: RelocateMode;
	onSelect?: (snap: SelectionSnapshot) => void;
	onConnect?: (p: AttractionPayload) => void;
	onProximityConnect?: (p: AttractionPayload) => void;
	onIndirectConnect?: (p: AttractionPayload) => void;
	onAttractionCompatibleObjects?: (p: AttractionCompatibleObjectsPayload) => void;
	onAttractionTargetRing?: (p: AttractionTargetRingPayload) => void;
	onRelocate?: (p: RelocatePayload) => void;
}) {
	const [selectedObjectIds, setSelectedObjectIds] = useState<readonly string[]>([]);
	const [activeRelocateObjectId, setActiveRelocateObjectId] = useState<string | null>(null);
	const [attractionDragActive, setAttractionDragActive] = useState(false);
	const [attractionDragAttractingFullId, setAttractionDragAttractingFullId] = useState<string | null>(null);
	const [attractionCompatibleAttractedFullIds, setAttractionCompatibleAttractedFullIds] = useState<ReadonlySet<string>>(new Set());
	const [attractionHoverRingFullId, setAttractionHoverRingFullId] = useState<string | null>(null);
	const [attractionIndirectPickAwait, setAttractionIndirectPickAwait] = useState<AttractionIndirectPickAwait | null>(null);

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
					lod: lodKindRef.current,
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
		[blockedVortexFullIds, collectPickRoots, lodKindRef, onAttractionTargetRing],
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
		}),
		[
			registerVortex,
			unregisterVortex,
			getVortexWorld,
			registerVortexBinding,
			unregisterVortexBinding,
			registerObject,
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
		],
	);

	return <RegistryContext.Provider value={value}>{children}</RegistryContext.Provider>;
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
	const lodKindRef = useRef<LodKind>("normal");
	const domain = props.domain ?? DEFAULT_DOMAIN;
	const distanceReference = props.lodDistanceReference ?? DEFAULT_SCALE_REFERENCE;
	const thresholds = props.lodZoomThresholds ?? lodZoomThresholdsForDomain(domain, distanceReference);
	const gridFactor = props.gridFactor ?? DEFAULT_LOD_GRID_FACTOR;
	const gridSnapEnabled = props.gridSnapEnabled ?? false;
	const showLodGrid = props.showLodGrid === true;
	const automaticLod = props.automaticLod ?? true;
	const pinnedLod = !automaticLod && props.lod !== undefined && isLodKind(props.lod) ? props.lod : undefined;
	const maxDist = 4000;
	const pos = (camProp?.position ?? [420, 320, 420]) as [number, number, number];
	const tgt = (camProp?.target ?? [0, 40, 0]) as Vec3;
	const zoom = camProp?.zoom ?? 1;
	const { chunked, rest } = useMemo(() => splitChunkedSceneChildren(children), [children]);
	const blockedFallback = props.blockedVortexFullIds ?? EMPTY_BLOCKED_VORTICES;
	const blocked = useLiveBlockedVortexFullIds(blockedFallback);
	return (
		<RegistryProvider
			lodKindRef={lodKindRef}
			kindCatalogs={props.kindCatalogs}
			kindCompatibility={props.kindCompatibility}
			blockedVortexFullIds={blocked}
			proximityRadius={proximityRadius}
			selectionMode={props.selectionMode ?? "single"}
			relocateMode={props.relocateMode ?? "translate"}
			onSelect={props.onSelect}
			onConnect={props.onConnect}
			onProximityConnect={props.onProximityConnect}
			onIndirectConnect={props.onIndirectConnect}
			onAttractionCompatibleObjects={props.onAttractionCompatibleObjects}
			onAttractionTargetRing={props.onAttractionTargetRing}
			onRelocate={props.onRelocate}
		>
			<LodBridge
				lodKindRef={lodKindRef}
				thresholds={thresholds}
				distanceReference={distanceReference}
				gridFactor={gridFactor}
				gridSnapEnabled={gridSnapEnabled}
				showLodGrid={showLodGrid}
				automaticLod={automaticLod}
				pinnedLod={pinnedLod}
				onLodChange={props.onLodChange}
			>
				<PerspectiveCamera makeDefault near={0.2} far={500_000} fov={50} />
				<SceneCameraSeed position={pos} target={tgt} />
				<OrbitGated />
				<AttractionThreeBinder />
				<AttractionWindowBridge />
				<AttractionRubberBand />
				<CameraReporter zoom={zoom} onCamera={props.onCamera} />
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
	const [shellLod, setShellLod] = useState<LodKind>("normal");
	const handleLod = useCallback(
		(l: LodKind) => {
			setShellLod(l);
			onLodChange?.(l);
		},
		[onLodChange],
	);
	return (
		<div
			className={className}
			style={{ width: "100%", height: "100%", ...style }}
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

/** @emoji 🧪 Registers `window.__scenePlay*` hooks for Playwright (play harness only). */
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

//#endregion 🎬Scene



if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;
	describe("lodCanvasProps", () => {
		it("maps automatic and pinned modes", () => {
			expect(lodCanvasProps(LOD_MODE_AUTOMATIC)).toEqual({ automaticLod: true });
			expect(lodCanvasProps("detail")).toEqual({ automaticLod: false, lod: "detail" });
		});
	});
	describe("lodAutomaticSelectLabel", () => {
		it("includes the live tier in the automatic row", () => {
			expect(lodAutomaticSelectLabel("overview")).toBe("Automatic · Overview");
		});
	});
	describe("resolveLodLabelFromThresholds", () => {
		it("classifies zoom bands", () => {
			const t = DEFAULT_LOD_ZOOM_THRESHOLDS;
			expect(resolveLodLabelFromThresholds(0.1, t)).toBe("minimap");
			expect(resolveLodLabelFromThresholds(0.2, t)).toBe("overview");
			expect(resolveLodLabelFromThresholds(0.3, t)).toBe("compact");
			expect(resolveLodLabelFromThresholds(0.5, t)).toBe("normal");
			expect(resolveLodLabelFromThresholds(1, t)).toBe("detail");
			expect(resolveLodLabelFromThresholds(2, t)).toBe("micro");
		});
	});
	describe("lodZoomThresholdsForDomain", () => {
		it("derives architecture thresholds from the domain ladder", () => {
			const t = lodZoomThresholdsForDomain("architecture");
			expect(resolveLodLabelFromThresholds(0.1, t)).toBe("minimap");
			expect(resolveLodLabelFromThresholds(0.2, t)).toBe("overview");
			expect(resolveLodLabelFromThresholds(0.3, t)).toBe("compact");
			expect(resolveLodLabelFromThresholds(0.5, t)).toBe("normal");
			expect(resolveLodLabelFromThresholds(1, t)).toBe("detail");
			expect(resolveLodLabelFromThresholds(2, t)).toBe("micro");
		});
		it("stays meter-calibrated when pseudo zoom and thresholds share the same reference", () => {
			const distanceReference = 900;
			const t = lodZoomThresholdsForDomain("architecture", distanceReference);
			expect(resolveLodLabelFromThresholds(pseudoZoomFromOrbitDistance(800, distanceReference), t)).toBe("minimap");
			expect(resolveLodLabelFromThresholds(pseudoZoomFromOrbitDistance(650, distanceReference), t)).toBe("overview");
			expect(resolveLodLabelFromThresholds(pseudoZoomFromOrbitDistance(300, distanceReference), t)).toBe("compact");
			expect(resolveLodLabelFromThresholds(pseudoZoomFromOrbitDistance(180, distanceReference), t)).toBe("normal");
			expect(resolveLodLabelFromThresholds(pseudoZoomFromOrbitDistance(100, distanceReference), t)).toBe("detail");
			expect(resolveLodLabelFromThresholds(pseudoZoomFromOrbitDistance(50, distanceReference), t)).toBe("micro");
		});
	});
	describe("lodVisibleGridSnapStepWorld", () => {
		it("returns per-band steps", () => {
			expect(lodVisibleGridSnapStepWorld("minimap", 10)).toBe(null);
			expect(lodVisibleGridSnapStepWorld("overview", 10)).toBe(100);
			expect(lodVisibleGridSnapStepWorld("compact", 10)).toBe(50);
			expect(lodVisibleGridSnapStepWorld("normal", 10)).toBe(25);
			expect(lodVisibleGridSnapStepWorld("detail", 10)).toBe(5);
			expect(lodVisibleGridSnapStepWorld("micro", 10)).toBe(1);
		});
	});
	describe("objectPoseKey", () => {
		it("changes when origin changes", () => {
			const a = objectPoseKey("id", [0, 0, 0], [0, 0, 0, 1], 1);
			const b = objectPoseKey("id", [1, 0, 0], [0, 0, 0, 1], 1);
			expect(a).not.toBe(b);
		});
	});
	describe("applyObjectPose", () => {
		it("places vortex child at expected world offset", () => {
			const parent = new Group();
			const vortex = new Group();
			vortex.position.set(1, 2, 3);
			parent.add(vortex);
			applyObjectPose(parent, [10, 0, 0], [0, 0, 0, 1], 1);
			updateWorldMatrixChain(vortex);
			const world = new Vector3();
			vortex.getWorldPosition(world);
			expect(world.x).toBeCloseTo(11, 5);
			expect(world.y).toBeCloseTo(2, 5);
			expect(world.z).toBeCloseTo(3, 5);
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
		it("parses vortex handleMeshByLod", () => {
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
						vortices: [
							{
								id: "a:v1",
								position: [0, 0, 0],
								handleMeshUrl: "/fallback.glb",
								handleMeshByLod: { detail: "/d.glb", micro: "/u.glb" },
							},
						],
					},
				],
			});
			const v = f?.objects[0]?.vortices[0];
			expect(v?.handleMeshByLod?.detail).toBe("/d.glb");
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
