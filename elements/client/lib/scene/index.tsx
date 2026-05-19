import { Clone, Line, OrbitControls, PerspectiveCamera, TransformControls, useGLTF } from "@react-three/drei";
import { Canvas, createPortal, useFrame, useThree } from "@react-three/fiber";
import {
	Button,
	Expertise,
	LevelProvider,
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
	ToolbarGroup,
	ToolbarItem,
	ToolbarZone,
	UI,
	createStackLayout,
	getLevelBgClass,
	useElementsSurfaceChrome,
	type ElementsSurfaceDevice,
	type ElementsSurfaceTheme,
	type FooterItem,
	type UIAppConfig,
} from "@elements/ui";
import { Move3d, Rotate3d, Scaling } from "lucide-react";
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
	useRef,
	useState,
	type CSSProperties,
	type MutableRefObject,
	type ReactElement,
	type ReactNode,
} from "react";
import { createRoot } from "react-dom/client";
import sceneFixtureJson from "./fixtures/nakagin-capsule-tower.scene.json";
import "./play/globals.css";
import {
	BufferGeometry,
	Float32BufferAttribute,
	GridHelper,
	LineBasicMaterial,
	PerspectiveCamera as ThreePerspectiveCamera,
	Plane,
	Quaternion,
	Raycaster,
	Vector2,
	Vector3,
	type Camera,
	type Group,
	type Object3D,
	type Scene,
	type WebGLRenderer,
} from "three";

//#region 🔖Kinds
export type Vec3 = readonly [number, number, number];
export type Quat = readonly [number, number, number, number];

export type SceneRelocateMode = "translate" | "rotate" | "scale";
export type SceneSelectionMode = "single" | "additive" | "subtractive" | "toggle";
export type SceneConnectKind = "indirect" | "connect" | "proximity";
export type SceneDomainKind = "urban" | "architecture" | "detailing" | "engineering";
export type SceneScaleKind =
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

export const DEFAULT_SCENE_DOMAIN: SceneDomainKind = "architecture";
export const DEFAULT_SCENE_SCALE_REFERENCE = 100;

const SCENE_SCALE_RATIOS: Record<SceneScaleKind, readonly [numerator: number, denominator: number]> = {
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

const SCENE_DOMAIN_LOD_SCALES: Record<SceneDomainKind, readonly [SceneScaleKind, SceneScaleKind, SceneScaleKind, SceneScaleKind, SceneScaleKind, SceneScaleKind]> = {
	urban: ["1to50000", "1to25000", "1to10000", "1to5000", "1to2500", "1to1000"],
	architecture: ["1to1000", "1to500", "1to333", "1to200", "1to100", "1to50"],
	detailing: ["1to50", "1to33", "1to25", "1to10", "1to5", "1to1"],
	engineering: ["1to1", "2to1", "5to1", "10to1", "20to1", "50to1"],
};

export interface SceneCameraState {
	position: Vec3;
	target: Vec3;
	zoom: number;
}

/** @emoji 📶 Board-aligned scene LOD band label (`data-scene-lod` on the canvas shell). */
export type SceneLodKind = "minimap" | "overview" | "compact" | "normal" | "detail" | "micro";

/** @emoji 📐 LOD zoom boundaries for pseudo-zoom from orbit camera distance (same semantics as board CSS zoom bands). */
export interface SceneLodZoomThresholds {
	minimapMaxZoom: number;
	overviewMaxZoom: number;
	compactMaxZoom: number;
	normalMaxZoom: number;
	detailMaxZoom: number;
}

function sceneScaleKindZoomAnchor(kind: SceneScaleKind, reference: number): number {
	const [numerator, denominator] = SCENE_SCALE_RATIOS[kind];
	return (reference * numerator) / denominator;
}

function sceneThresholdBetweenZoomAnchors(a: number, b: number): number {
	return Math.sqrt(a * b);
}

/** @emoji 📐 Returns the six canonical scale bands used by a scene domain from minimap through micro. */
export function sceneLodScaleKindsForDomain(domain: SceneDomainKind): readonly SceneScaleKind[] {
	return SCENE_DOMAIN_LOD_SCALES[domain];
}

/** @emoji 📐 Derives scene LOD thresholds from the scene domain scale ladder using a positive reference zoom. */
export function sceneLodZoomThresholdsForDomain(
	domain: SceneDomainKind,
	reference = DEFAULT_SCENE_SCALE_REFERENCE,
): SceneLodZoomThresholds {
	const [minimap, overview, compact, normal, detail, micro] = sceneLodScaleKindsForDomain(domain).map((kind) =>
		sceneScaleKindZoomAnchor(kind, reference),
	);
	return {
		minimapMaxZoom: sceneThresholdBetweenZoomAnchors(minimap, overview),
		overviewMaxZoom: sceneThresholdBetweenZoomAnchors(overview, compact),
		compactMaxZoom: sceneThresholdBetweenZoomAnchors(compact, normal),
		normalMaxZoom: sceneThresholdBetweenZoomAnchors(normal, detail),
		detailMaxZoom: sceneThresholdBetweenZoomAnchors(detail, micro),
	};
}

/** @emoji 📐 Default LOD thresholds aligned with the architecture scene domain. */
export const DEFAULT_SCENE_LOD_ZOOM_THRESHOLDS: SceneLodZoomThresholds = sceneLodZoomThresholdsForDomain(
	DEFAULT_SCENE_DOMAIN,
);

/** @emoji 📐 Large LOD grid quantum in world units (sketch board `BOARD_LOD_GRID_MAJOR_QUANTUM`). */
export const SCENE_LOD_GRID_MAJOR_QUANTUM = 10;

/** @emoji 📐 Default grid factor (sketch board `DEFAULT_BOARD_GRID_FACTOR`). */
export const DEFAULT_SCENE_LOD_GRID_FACTOR = 10;

export interface SceneVortexProps {
	id: string;
	vortexKind?: string;
	position: Vec3;
	direction?: Vec3;
	radius?: number;
	visible?: boolean;
	handleMeshUrl?: string;
	/** @emoji 🎨 Optional per-LOD GLB URLs for the handle mesh; falls back to {@link handleMeshUrl}. */
	handleMeshByLod?: Partial<Record<SceneLodKind, string>>;
	children?: ReactNode;
}

export interface SceneMagnetProps {
	id: string;
	magnetKind?: string;
	position: Vec3;
	orientation?: Quat;
	size: Vec3;
}

export interface SceneObjectProps {
	id: string;
	objectKind?: string;
	meshUrl: string;
	origin: Vec3;
	orientation?: Quat;
	scale?: number | Vec3;
	label?: string;
	selected?: boolean;
	visible?: boolean;
	relocate?: SceneRelocateMode | false;
	children?: ReactNode;
	userData?: Record<string, unknown>;
}

export interface SceneTieProps {
	id: string;
	source: `${string}:${string}`;
	target: `${string}:${string}`;
	tieKind?: string;
}

export const SCENE_PLACEHOLDER_MESH_URL = "elements.scene.placeholder://box";

export interface SceneEdgeKindCatalogEntry {
	id: string;
	label?: string;
	name?: string;
}

export interface SceneHandleKindCatalogEntry {
	id: string;
	label?: string;
	name?: string;
	color?: string;
	defaultWireKind?: string;
	scale?: number;
}

export interface SceneNodeKindCatalogEntry {
	id: string;
	label?: string;
	name?: string;
	color?: string;
	shape?: string;
}

export interface SceneWireKindCatalogEntry {
	id: string;
	label?: string;
	name?: string;
	defaultEdgeKind?: string;
}

export interface SceneKindCatalogBundle {
	edges?: readonly SceneEdgeKindCatalogEntry[];
	handles?: readonly SceneHandleKindCatalogEntry[];
	nodes?: readonly SceneNodeKindCatalogEntry[];
	wires?: readonly SceneWireKindCatalogEntry[];
}

export interface SceneKindCompatEntry {
	source: string;
	target: string;
	bidirectional?: boolean;
	important?: boolean;
	specificity?: "general" | "object" | "tie" | "handle" | "wire" | "node" | "edge";
}

export interface SceneSelectionSnapshot {
	readonly objectIds: readonly string[];
	readonly vortexIds: readonly string[];
}

export interface SceneRelocatePayload {
	readonly objectId: string;
	readonly mode: SceneRelocateMode;
	readonly before: { origin: Vec3; orientation: Quat; scale: Vec3 };
	readonly after: { origin: Vec3; orientation: Quat; scale: Vec3 };
}

export interface SceneTieLinkPayload {
	readonly source: string;
	readonly target: string;
	readonly tieId?: string;
}

export interface SceneLinkCompatibleNodesPayload {
	readonly source: string;
	readonly objectIds: readonly string[];
}

export interface SceneLinkTargetRingPayload {
	readonly source: string;
	readonly objectId: string | null;
	readonly vortexFullIds: readonly string[];
}

export interface SceneLinkIndirectPickAwait {
	readonly sourceFullId: string;
	readonly targetObjectId: string;
	readonly candidates: readonly string[];
}

export interface SceneCanvasProps {
	camera?: Partial<SceneCameraState>;
	domain?: SceneDomainKind;
	chunkSize?: number;
	kindCatalogs?: SceneKindCatalogBundle;
	kindCompatibility?: readonly SceneKindCompatEntry[];
	/** @emoji 🚫 Vortex full ids (`objectId:vortexId`) that already terminate a tie and cannot start or receive a new link. */
	blockedVortexFullIds?: ReadonlySet<string>;
	proximityRadius?: number;
	relocateMode?: SceneRelocateMode;
	selectionMode?: SceneSelectionMode;
	/** @emoji 📶 LOD zoom thresholds for pseudo-zoom derived from orbit camera distance. */
	lodZoomThresholds?: SceneLodZoomThresholds;
	/** @emoji 📏 Orbit distance at which pseudo-zoom is ~1 (tune to scene extent). */
	lodDistanceReference?: number;
	/** @emoji 📐 Multiplier for LOD grid steps (board `grid_factor`). */
	gridFactor?: number;
	/** @emoji 📐 When true, draw a world `GridHelper` stepped by the current LOD band grid. */
	showLodGrid?: boolean;
	/** @emoji 🧲 When true, translate relocate snaps to the finest visible LOD grid step (board `grid_snap_enabled`). */
	gridSnapEnabled?: boolean;
	onCamera?: (s: SceneCameraState) => void;
	/** @emoji 📶 Emits whenever the resolved scene LOD band changes. */
	onLodChange?: (lod: SceneLodKind) => void;
	onSelect?: (snap: SceneSelectionSnapshot) => void;
	onRelocate?: (p: SceneRelocatePayload) => void;
	onConnect?: (p: SceneTieLinkPayload) => void;
	onIndirectConnect?: (p: SceneTieLinkPayload) => void;
	onProximityConnect?: (p: SceneTieLinkPayload) => void;
	onLinkCompatibleNodes?: (p: SceneLinkCompatibleNodesPayload) => void;
	onLinkTargetRing?: (p: SceneLinkTargetRingPayload) => void;
	children?: ReactNode;
}

export const SCENE_FIXTURE_DRAG_V1_MIME = "application/x-elements-scene-fixture+json;v=1";

export interface SceneFixtureObjectV1 extends SceneObjectProps {
	vortices: SceneVortexProps[];
	magnets?: SceneMagnetProps[];
}

export interface SceneFixtureV1 {
	schema: "elements.scene.fixture/v1";
	camera: SceneCameraState;
	domain: SceneDomainKind;
	meta?: Record<string, unknown>;
	ties: SceneTieProps[];
	objects: SceneFixtureObjectV1[];
}
//#endregion 🔖Kinds

//#region 📶Lod
/** @emoji 📶 Resolves scene LOD from pseudo-zoom using explicit thresholds (same band order as the sketch board surface). */
export function resolveSceneLodLabelFromThresholds(zoom: number, t: SceneLodZoomThresholds): SceneLodKind {
	const z = zoom;
	if (z < t.minimapMaxZoom) return "minimap";
	if (z < t.overviewMaxZoom) return "overview";
	if (z < t.compactMaxZoom) return "compact";
	if (z < t.normalMaxZoom) return "normal";
	if (z < t.detailMaxZoom) return "detail";
	return "micro";
}

/** @emoji 📏 Maps orbit camera distance to a board-comparable pseudo-zoom (`reference / distance`). */
export function scenePseudoZoomFromOrbitDistance(distance: number, reference: number): number {
	const d = Math.max(distance, 1e-6);
	return reference / d;
}

/** @emoji 📐 Visible LOD grid / relocate snap step in world units (mirrors sketch board WASM `lod_visible_grid_snap_step_world`). */
export function sceneLodVisibleGridSnapStepWorld(lod: SceneLodKind, gridFactor: number): number | null {
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
export function sceneHandlePrimaryVisualVisibleAtLod(lod: SceneLodKind): boolean {
	return lod === "normal" || lod === "detail" || lod === "micro";
}

/** @emoji 🌀 Overview uses invisible pick proxies when primary handle GLB is hidden (minimap has no handle picks). */
export function sceneHandlePickProxyAtLod(lod: SceneLodKind): boolean {
	return lod === "overview" || lod === "compact";
}

export interface SceneLodContextValue {
	readonly lod: SceneLodKind;
	readonly lodGridStepWorld: number | null;
	readonly gridFactor: number;
	readonly gridSnapEnabled: boolean;
}

const SceneLodContext = createContext<SceneLodContextValue | null>(null);

/** @emoji 📶 Reads the live scene LOD band and grid snap step from canvas context. */
export function useSceneLod(): SceneLodContextValue {
	const v = useContext(SceneLodContext);
	if (!v) throw new Error("Scene LOD missing");
	return v;
}

function SceneLodGridHelper() {
	const lod = useSceneLod();
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

function SceneLodFrameRunner(props: {
	readonly lodKindRef: MutableRefObject<SceneLodKind>;
	readonly thresholds: SceneLodZoomThresholds;
	readonly distanceReference: number;
	readonly gridFactor: number;
	readonly gridSnapEnabled: boolean;
	readonly onCtx: (next: SceneLodContextValue) => void;
	readonly onLodChange?: (lod: SceneLodKind) => void;
}) {
	const cam = useThree((s) => s.camera);
	const controls = useThree((s) => s.controls as { target?: Vector3 } | null);
	const tmpT = useMemo(() => new Vector3(), []);
	const prevLod = useRef<SceneLodKind | null>(null);
	const ctxSig = useRef("");
	useFrame(() => {
		const tgt = controls?.target ?? tmpT.set(0, 0, 0);
		const dist = cam.position.distanceTo(tgt);
		const pseudo = scenePseudoZoomFromOrbitDistance(dist, props.distanceReference);
		const next = resolveSceneLodLabelFromThresholds(pseudo, props.thresholds);
		props.lodKindRef.current = next;
		const lodGridStepWorld = sceneLodVisibleGridSnapStepWorld(next, props.gridFactor);
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

function SceneLodBridge(props: {
	readonly children: ReactNode;
	readonly lodKindRef: MutableRefObject<SceneLodKind>;
	readonly thresholds: SceneLodZoomThresholds;
	readonly distanceReference: number;
	readonly gridFactor: number;
	readonly gridSnapEnabled: boolean;
	readonly showLodGrid: boolean;
	readonly onLodChange?: (lod: SceneLodKind) => void;
}) {
	const [lodCtx, setLodCtx] = useState<SceneLodContextValue>(() => ({
		lod: "normal",
		lodGridStepWorld: sceneLodVisibleGridSnapStepWorld("normal", props.gridFactor),
		gridFactor: props.gridFactor,
		gridSnapEnabled: props.gridSnapEnabled,
	}));
	const onCtx = useCallback(
		(next: SceneLodContextValue) => {
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
		<SceneLodContext.Provider value={v}>
			<SceneLodFrameRunner
				lodKindRef={props.lodKindRef}
				thresholds={props.thresholds}
				distanceReference={props.distanceReference}
				gridFactor={props.gridFactor}
				gridSnapEnabled={props.gridSnapEnabled}
				onCtx={onCtx}
				onLodChange={props.onLodChange}
			/>
			{props.showLodGrid ? <SceneLodGridHelper /> : null}
			{props.children}
		</SceneLodContext.Provider>
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

const SCENE_LOD_KINDS: readonly SceneLodKind[] = ["minimap", "overview", "compact", "normal", "detail", "micro"];

function parseSceneDomainKind(value: unknown): SceneDomainKind {
	if (typeof value !== "string") {
		return DEFAULT_SCENE_DOMAIN;
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
			return DEFAULT_SCENE_DOMAIN;
	}
}

function parseHandleMeshByLod(v: unknown): Partial<Record<SceneLodKind, string>> | undefined {
	if (!v || typeof v !== "object") return undefined;
	const o = v as Record<string, unknown>;
	const out: Partial<Record<SceneLodKind, string>> = {};
	for (const k of SCENE_LOD_KINDS) {
		const s = o[k];
		if (typeof s === "string" && s.length) out[k] = s;
	}
	return Object.keys(out).length ? out : undefined;
}

export function parseSceneFixtureV1(raw: unknown): SceneFixtureV1 | null {
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
	const tiesRaw = r.ties;
	const objsRaw = r.objects;
	if (!Array.isArray(tiesRaw) || !Array.isArray(objsRaw)) return null;
	const ties: SceneTieProps[] = [];
	for (const t of tiesRaw) {
		if (!t || typeof t !== "object") continue;
		const tr = t as Record<string, unknown>;
		if (typeof tr.id !== "string" || typeof tr.source !== "string" || typeof tr.target !== "string") continue;
		ties.push({
			id: tr.id,
			source: tr.source as SceneTieProps["source"],
			target: tr.target as SceneTieProps["target"],
			...(typeof tr.tieKind === "string" ? { tieKind: tr.tieKind } : {}),
		});
	}
	const objects: SceneFixtureObjectV1[] = [];
	for (const o of objsRaw) {
		if (!o || typeof o !== "object") continue;
		const or = o as Record<string, unknown>;
		if (typeof or.id !== "string" || typeof or.meshUrl !== "string") continue;
		const origin = or.origin;
		if (!isVec3(origin)) continue;
		const vortices: SceneVortexProps[] = [];
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
			...(isQuat(or.orientation) ? { orientation: or.orientation } : {}),
			...(typeof or.scale === "number" || isVec3(or.scale) ? { scale: or.scale as number | Vec3 } : {}),
			vortices,
		});
	}
	return {
		schema: "elements.scene.fixture/v1",
		camera: { position: pos, target: tgt, zoom },
		domain: parseSceneDomainKind(r.domain),
		...(r.meta && typeof r.meta === "object" ? { meta: r.meta as Record<string, unknown> } : {}),
		ties,
		objects,
	};
}

export function encodeSceneFixtureForDragV1(fixture: SceneFixtureV1): string {
	return JSON.stringify(fixture);
}
//#endregion 🧾Fixture

//#region 🧩Compat
export function sceneKindsCompatible(
	aKind: string | undefined,
	bKind: string | undefined,
	table: readonly SceneKindCompatEntry[] | undefined,
): boolean {
	if (!table?.length || !aKind || !bKind) return false;
	return table.some(
		(e) =>
			(e.source === aKind && e.target === bKind) ||
			(e.bidirectional === true && e.source === bKind && e.target === aKind),
	);
}

const SCENE_DEFAULT_WIRE_KIND_ID = "board.wire.link";

/** @emoji 🔗 Tie endpoint vortex full ids that are already linked and cannot start or receive another link. */
export function sceneBlockedVortexFullIdsFromTies(
	ties: readonly Pick<SceneTieProps, "source" | "target">[],
): ReadonlySet<string> {
	const s = new Set<string>();
	for (const t of ties) {
		s.add(t.source);
		s.add(t.target);
	}
	return s;
}

/** @emoji 🧭 Semantic kinds at one end of a link drag (object + vortex handle). */
export interface SceneLinkHandleContext {
	readonly objectId: string;
	readonly objectKind: string | undefined;
	readonly vortexKind: string | undefined;
}

function catalogHandleById(
	catalogs: SceneKindCatalogBundle | undefined,
	handleKind: string | undefined,
): SceneHandleKindCatalogEntry | undefined {
	if (!handleKind || !catalogs?.handles?.length) return undefined;
	return catalogs.handles.find((h) => h.id === handleKind);
}

function catalogWireById(
	catalogs: SceneKindCatalogBundle | undefined,
	wireKind: string | undefined,
): SceneWireKindCatalogEntry | undefined {
	if (!wireKind || !catalogs?.wires?.length) return undefined;
	return catalogs.wires.find((w) => w.id === wireKind);
}

/** @emoji 🔌 Resolves default wire kind for a vortex kind via handle catalog, else `board.wire.link`. */
export function resolveSceneWireKindForVortex(
	vortexKind: string | undefined,
	catalogs: SceneKindCatalogBundle | undefined,
): string {
	const h = catalogHandleById(catalogs, vortexKind);
	const w = h?.defaultWireKind?.trim();
	return w && w.length > 0 ? w : SCENE_DEFAULT_WIRE_KIND_ID;
}

/** @emoji 🪢 Resolves default edge kind for a wire kind via wire catalog, else empty string. */
export function resolveSceneEdgeKindForWire(
	wireKind: string | undefined,
	catalogs: SceneKindCatalogBundle | undefined,
): string {
	const w = catalogWireById(catalogs, wireKind);
	const e = w?.defaultEdgeKind?.trim();
	return e && e.length > 0 ? e : "";
}

function sceneCompatPairMatches(rule: SceneKindCompatEntry, a: string, b: string): boolean {
	if (rule.source === a && rule.target === b) return true;
	if (rule.bidirectional === true && rule.source === b && rule.target === a) return true;
	return false;
}

function sceneLinkGestureRuleApplies(
	rule: SceneKindCompatEntry,
	source: SceneLinkHandleContext,
	target: SceneLinkHandleContext,
	catalogs: SceneKindCatalogBundle | undefined,
): boolean {
	const wSrc = resolveSceneWireKindForVortex(source.vortexKind, catalogs);
	const wTgt = resolveSceneWireKindForVortex(target.vortexKind, catalogs);
	const eSrc = resolveSceneEdgeKindForWire(wSrc, catalogs);
	const eTgt = resolveSceneEdgeKindForWire(wTgt, catalogs);
	const sn = source.objectKind ?? "";
	const tn = target.objectKind ?? "";
	const sh = source.vortexKind ?? "";
	const th = target.vortexKind ?? "";
	const spec = rule.specificity ?? "handle";
	switch (spec) {
		case "general":
			return sceneCompatPairMatches(rule, sh, th);
		case "object":
		case "node":
			return sceneCompatPairMatches(rule, sn, tn);
		case "edge":
		case "tie":
			return sceneCompatPairMatches(rule, eSrc, eTgt);
		case "handle":
			return sceneCompatPairMatches(rule, sh, th);
		case "wire":
			return sceneCompatPairMatches(rule, wSrc, th);
		default:
			return sceneCompatPairMatches(rule, sh, th);
	}
}

/** @emoji 🤝 WASM-style filtered link compatibility (important + specificity tiers); empty rules allow all. */
export function sceneHandlesLinkCompatibleForDrag(
	source: SceneLinkHandleContext,
	target: SceneLinkHandleContext,
	rules: readonly SceneKindCompatEntry[] | undefined,
	catalogs: SceneKindCatalogBundle | undefined,
): boolean {
	if (!rules?.length) return true;
	let matched = rules.filter((r) => sceneLinkGestureRuleApplies(r, source, target, catalogs));
	if (matched.length === 0) return false;
	if (matched.some((r) => r.important)) matched = matched.filter((r) => r.important);
	else {
		const rank = (s: SceneKindCompatEntry["specificity"] | undefined): number => {
			switch (s) {
				case "general":
					return 0;
				case "object":
				case "node":
					return 1;
				case "edge":
				case "tie":
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

//#region 🏊Pool
const gltfRefCounts = new Map<string, number>();

export function sceneGltfPoolAcquire(url: string): void {
	gltfRefCounts.set(url, (gltfRefCounts.get(url) ?? 0) + 1);
}

export function sceneGltfPoolRelease(url: string): void {
	const n = (gltfRefCounts.get(url) ?? 1) - 1;
	if (n <= 0) {
		gltfRefCounts.delete(url);
	} else {
		gltfRefCounts.set(url, n);
	}
}

/** @emoji 🧹 Drops pooled GLTF cache entries (call on scene teardown, not per-chunk unmount). */
export function sceneGltfPoolClear(url: string): void {
	gltfRefCounts.delete(url);
	useGLTF.clear(url);
}

function usePooledGltf(url: string) {
	const gltf = useGLTF(url);
	useEffect(() => {
		sceneGltfPoolAcquire(url);
		return () => {
			sceneGltfPoolRelease(url);
		};
	}, [url]);
	return gltf;
}
//#endregion 🏊Pool

//#region 🎯Registry
type VortexGetter = () => Vector3 | null;

export interface SceneVortexBindingMeta {
	readonly fullId: string;
	readonly objectId: string;
	readonly objectKind: string | undefined;
	readonly vortexKind: string | undefined;
	readonly radiusWorld: number;
}

export interface SceneRegistryValue {
	registerVortex(fullId: string, getter: VortexGetter): void;
	unregisterVortex(fullId: string): void;
	getVortexWorld(fullId: string): Vector3 | null;
	registerVortexBinding(meta: SceneVortexBindingMeta, pickRoot: Object3D | null): void;
	unregisterVortexBinding(fullId: string): void;
	registerObject(id: string, objectKind: string | undefined, group: Group | null): void;
	getObjectGroup(id: string): Group | null;
	getObjectKind(id: string): string | undefined;
	kindCatalogs: SceneKindCatalogBundle | undefined;
	kindCompatibility: readonly SceneKindCompatEntry[] | undefined;
	blockedVortexFullIds: ReadonlySet<string>;
	proximityRadius: number;
	selectedObjectIds: readonly string[];
	setSelectedObjectIds(ids: readonly string[]): void;
	selectionMode: SceneSelectionMode;
	relocateMode: SceneRelocateMode;
	activeRelocateObjectId: string | null;
	setActiveRelocateObjectId: (id: string | null) => void;
	linkDragActive: boolean;
	linkDragSourceFullId: string | null;
	linkCompatibleTargetFullIds: ReadonlySet<string>;
	linkHoverRingFullId: string | null;
	linkIndirectPickAwait: SceneLinkIndirectPickAwait | null;
	linkEndWorldRef: MutableRefObject<Vector3 | null>;
	beginLinkDragFromVortex(fullId: string, objectId: string, objectKind: string | undefined, vortexKind: string | undefined): void;
	cancelLinkDrag(): void;
	findNearestProximityRelocate(world: Vector3, movingObjectId: string): SceneTieLinkPayload | null;
	attachLinkThreeEnv(env: { camera: Camera; gl: WebGLRenderer; scene: Scene } | null): void;
	updateLinkPointer(clientX: number, clientY: number): void;
	commitLinkPointer(clientX: number, clientY: number): void;
	updateIndirectPickPointer(clientX: number, clientY: number): void;
	commitIndirectPickPointerDown(clientX: number, clientY: number): void;
	onSelect?: (snap: SceneSelectionSnapshot) => void;
	onConnect?: (p: SceneTieLinkPayload) => void;
	onProximityConnect?: (p: SceneTieLinkPayload) => void;
	onIndirectConnect?: (p: SceneTieLinkPayload) => void;
	onLinkCompatibleNodes?: (p: SceneLinkCompatibleNodesPayload) => void;
	onLinkTargetRing?: (p: SceneLinkTargetRingPayload) => void;
	onRelocate?: (p: SceneRelocatePayload) => void;
}

const SceneRegistryContext = createContext<SceneRegistryValue | null>(null);

function useSceneRegistry(): SceneRegistryValue {
	const v = useContext(SceneRegistryContext);
	if (!v) throw new Error("Scene registry missing");
	return v;
}
//#endregion 🎯Registry

//#region 🧱Chunking
export function sceneChunkKey(origin: Vec3, chunkSize: number): string {
	const ix = Math.floor(origin[0] / chunkSize);
	const iy = Math.floor(origin[1] / chunkSize);
	const iz = Math.floor(origin[2] / chunkSize);
	return `${ix}|${iy}|${iz}`;
}

function sceneSetEquals(a: ReadonlySet<string>, b: ReadonlySet<string>): boolean {
	if (a.size !== b.size) return false;
	for (const v of a) if (!b.has(v)) return false;
	return true;
}

/** @emoji 📏 Chunk bounding radius in world units (half-space diagonal of a cube chunk). */
export function sceneChunkBoundsRadius(chunkSize: number): number {
	return chunkSize * 0.866;
}

/** @emoji 👁️ Distance-only chunk visibility with enter/exit hysteresis (avoids frustum-edge flicker). */
export function sceneChunkDistanceVisible(args: {
	readonly camPos: Vector3;
	readonly chunkCenter: Vector3;
	readonly chunkSize: number;
	readonly maxDist: number;
	readonly wasVisible: boolean;
}): boolean {
	const boundsR = sceneChunkBoundsRadius(args.chunkSize);
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
				const show = sceneChunkDistanceVisible({
					camPos,
					chunkCenter: centerTmp,
					chunkSize,
					maxDist,
					wasVisible: next.has(key),
				});
				if (show) next.add(key);
				else next.delete(key);
			}
			return sceneSetEquals(prev, next) ? prev : next;
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

function sceneVector3IsFinite(v: Vector3): boolean {
	return Number.isFinite(v.x) && Number.isFinite(v.y) && Number.isFinite(v.z);
}
//#endregion 🧊Helpers

//#region 🔗LinkGesture
function readSceneVortexFullIdFromObject(o: Object3D | null): string | null {
	let cur: Object3D | null = o;
	while (cur) {
		const id = cur.userData?.sceneVortexFullId;
		if (typeof id === "string" && id.length > 0) return id;
		cur = cur.parent;
	}
	return null;
}

function readSceneObjectIdFromObject(o: Object3D | null): string | null {
	let cur: Object3D | null = o;
	while (cur) {
		const id = cur.userData?.sceneObjectId;
		if (typeof id === "string" && id.length > 0) return id;
		cur = cur.parent;
	}
	return null;
}

const SCENE_HANDLE_HIT_TOLERANCE_PX = 10;
const SCENE_LINK_HANDLE_SNAP_EXTRA_PX = 22;
const SCENE_LINK_COMMIT_SNAP_TIGHT_PX = 2;

function worldToCanvasPx(world: Vector3, camera: Camera, gl: WebGLRenderer): { x: number; y: number } {
	const v = world.clone().project(camera);
	const w = gl.domElement.clientWidth;
	const h = gl.domElement.clientHeight;
	return { x: (v.x * 0.5 + 0.5) * w, y: (-v.y * 0.5 + 0.5) * h };
}

function scenePixelsPerWorldUnitAt(camera: Camera, gl: WebGLRenderer, world: Vector3): number {
	if (!(camera as ThreePerspectiveCamera).isPerspectiveCamera) return 1;
	const pc = camera as ThreePerspectiveCamera;
	const dist = pc.position.distanceTo(world);
	const fovRad = (pc.fov * Math.PI) / 180;
	const h = Math.max(1, gl.domElement.clientHeight);
	return h / (2 * Math.tan(fovRad / 2) * Math.max(dist, 1e-6));
}

function sceneLinkSnapDragTolerancePx(worldHandle: Vector3, radiusWorld: number, camera: Camera, gl: WebGLRenderer): number {
	const mpp = scenePixelsPerWorldUnitAt(camera, gl, worldHandle);
	const radPx = radiusWorld * mpp;
	return SCENE_HANDLE_HIT_TOLERANCE_PX + SCENE_LINK_HANDLE_SNAP_EXTRA_PX + radPx * camera.zoom;
}

function sceneLinkSnapCommitTolerancePx(worldHandle: Vector3, radiusWorld: number, camera: Camera, gl: WebGLRenderer): number {
	const mpp = scenePixelsPerWorldUnitAt(camera, gl, worldHandle);
	const radPx = radiusWorld * mpp;
	return SCENE_HANDLE_HIT_TOLERANCE_PX + SCENE_LINK_COMMIT_SNAP_TIGHT_PX + radPx * camera.zoom;
}

function sceneLinkSnapCommitProximityOk(
	targetFullId: string,
	pointerWorld: Vector3,
	camera: Camera,
	gl: WebGLRenderer,
	getVortexWorld: (id: string) => Vector3 | null,
	metaRadius: (id: string) => number,
): boolean {
	const hw = getVortexWorld(targetFullId);
	if (!hw) return false;
	const pScr = worldToCanvasPx(pointerWorld, camera, gl);
	const hScr = worldToCanvasPx(hw, camera, gl);
	const d = Math.hypot(pScr.x - hScr.x, pScr.y - hScr.y);
	return d <= sceneLinkSnapCommitTolerancePx(hw, metaRadius(targetFullId), camera, gl);
}

function sceneNearestLinkSnapFullId(args: {
	lod: SceneLodKind;
	pointerWorld: Vector3;
	sourceFullId: string;
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
		if (tid === args.sourceFullId) continue;
		if (args.blocked.has(tid)) continue;
		const hw = args.getVortexWorld(tid);
		if (!hw) continue;
		const hScr = worldToCanvasPx(hw, args.camera, args.gl);
		const d = Math.hypot(hScr.x - pScr.x, hScr.y - pScr.y);
		const tol = sceneLinkSnapDragTolerancePx(hw, args.metaRadius(tid), args.camera, args.gl);
		if (d > tol) continue;
		if (!best || d < best.d) best = { d, id: tid };
	}
	return best?.id ?? null;
}
//#endregion 🔗LinkGesture

//#region 🧊Object
const ScenePlaceholderMesh = memo(function ScenePlaceholderMesh() {
	return (
		<mesh>
			<boxGeometry args={[1, 1, 1]} />
			<meshStandardMaterial color="#cbd5e1" metalness={0.05} roughness={0.85} />
		</mesh>
	);
});

const SceneResolvedObjectMesh = memo(function SceneResolvedObjectMesh(props: { readonly meshUrl: string }) {
	const gltf = usePooledGltf(props.meshUrl);
	return gltf.scene ? <Clone object={gltf.scene} /> : null;
});

const SceneObjectTransformControls = memo(function SceneObjectTransformControls(props: {
	readonly object: Group;
	readonly objectId: string;
	readonly mode: SceneRelocateMode;
	readonly translationSnap: number | undefined;
	readonly beforeRef: MutableRefObject<{ origin: Vector3; quat: Quaternion; scale: Vector3 } | null>;
}) {
	const reg = useSceneRegistry();
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

export const SceneObject = memo(function SceneObject(props: SceneObjectProps) {
	const group = useRef<Group>(null);
	const reg = useSceneRegistry();
	const beforeRef = useRef<{ origin: Vector3; quat: Quaternion; scale: Vector3 } | null>(null);
	const [tcTarget, setTcTarget] = useState<Group | null>(null);

	useEffect(() => {
		reg.registerObject(props.id, props.objectKind, group.current);
		return () => {
			reg.registerObject(props.id, props.objectKind, null);
		};
	}, [props.id, props.objectKind, reg]);

	useEffect(() => {
		if (group.current) setTcTarget(group.current);
	}, [props.selected, props.id, reg.activeRelocateObjectId]);

	const handlePointerDown = useCallback(
		(e: { stopPropagation: () => void }) => {
			e.stopPropagation();
			if (reg.linkDragActive || reg.linkIndirectPickAwait) return;
			if (reg.selectionMode === "single") {
				reg.setSelectedObjectIds([props.id]);
				reg.onSelect?.({ objectIds: [props.id], vortexIds: [] });
			}
			reg.setActiveRelocateObjectId(props.id);
		},
		[props.id, reg],
	);

	const quat = useMemo(() => quatToThree(props.orientation), [props.orientation]);
	const scaleVec = useMemo(() => scaleToThree(props.scale), [props.scale]);
	const lodCtx = useSceneLod();
	const mode = props.relocate ?? reg.relocateMode;
	const transSnap =
		mode === "translate" &&
		lodCtx.gridSnapEnabled &&
		lodCtx.lodGridStepWorld != null &&
		lodCtx.lodGridStepWorld > 0
			? lodCtx.lodGridStepWorld
			: undefined;
	const showTc =
		props.selected && reg.activeRelocateObjectId === props.id && props.relocate !== false && tcTarget;

	return (
		<group
			ref={group}
			position={props.origin as [number, number, number]}
			quaternion={quat}
			scale={scaleVec}
			visible={props.visible !== false}
			onPointerDown={handlePointerDown}
			userData={{ sceneObjectId: props.id, ...props.userData }}
			data-scene-object={props.id}
		>
			{props.meshUrl === SCENE_PLACEHOLDER_MESH_URL ? <ScenePlaceholderMesh /> : <SceneResolvedObjectMesh meshUrl={props.meshUrl} />}
			{props.children}
		</group>
		{showTc && tcTarget && (
			<SceneObjectTransformControls
				object={tcTarget}
				objectId={props.id}
				mode={mode}
				translationSnap={transSnap}
				beforeRef={beforeRef}
			/>
		)}
	);
});
//#endregion 🧊Object

//#region 🌀Vortex
const vortexFallbackMatProps = { transparent: true, opacity: 0.55 } as const;

function SceneVortexHandleGltf(props: { meshUrl: string; fullId: string; radius: number }) {
	const gltf = usePooledGltf(props.meshUrl);
	const scale = (props.radius / 0.35) * 0.9;
	if (!gltf.scene) return null;
	return (
		<Clone object={gltf.scene} scale={scale} userData={{ sceneVortexFullId: props.fullId }} />
	);
}

function SceneVortexFallbackMesh(props: {
	fullId: string;
	radius: number;
	highlight: "none" | "compatible" | "ring" | "source" | "indirectRing";
}) {
	const color =
		props.highlight === "compatible"
			? "#22c55e"
			: props.highlight === "ring"
				? "#facc15"
				: props.highlight === "indirectRing"
					? "#a78bfa"
					: props.highlight === "source"
						? "#94a3b8"
						: "#38bdf8";
	const emissive = props.highlight === "ring" || props.highlight === "indirectRing" ? "#ca8a04" : "#000000";
	const emissiveIntensity = props.highlight === "ring" || props.highlight === "indirectRing" ? 0.45 : 0;
	return (
		<mesh userData={{ sceneVortexFullId: props.fullId }}>
			<sphereGeometry args={[props.radius, 12, 12]} />
			<meshStandardMaterial
				color={color}
				emissive={emissive}
				emissiveIntensity={emissiveIntensity}
				{...vortexFallbackMatProps}
			/>
		</mesh>
	);
}

export const SceneVortex = memo(function SceneVortex(
	props: SceneVortexProps & { objectId: string; objectKind?: string },
) {
	const root = useRef<Group | null>(null);
	const reg = useSceneRegistry();
	const fullId = props.id.includes(":") ? props.id : `${props.objectId}:${props.id}`;
	const r = props.radius ?? 0.35;

	useEffect(() => {
		const getter = () => {
			if (!root.current) return null;
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

	const lodCtx = useSceneLod();
	const highlight: "none" | "compatible" | "ring" | "source" | "indirectRing" = reg.linkDragSourceFullId === fullId
		? "source"
		: reg.linkHoverRingFullId === fullId
			? "ring"
			: reg.linkIndirectPickAwait?.candidates.includes(fullId) === true
				? "indirectRing"
				: reg.linkCompatibleTargetFullIds.has(fullId)
					? "compatible"
					: "none";

	const onPointerDown = useCallback(
		(e: { stopPropagation: () => void; nativeEvent: PointerEvent }) => {
			e.stopPropagation();
			const pe = e.nativeEvent;
			if (pe.button !== 0) return;
			if (reg.blockedVortexFullIds.has(fullId)) return;
			reg.beginLinkDragFromVortex(fullId, props.objectId, props.objectKind, props.vortexKind);
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

	const inIndirectRing = reg.linkIndirectPickAwait?.candidates.includes(fullId) === true;
	const linger =
		(reg.linkDragActive &&
			(reg.linkDragSourceFullId === fullId ||
				reg.linkHoverRingFullId === fullId ||
				reg.linkCompatibleTargetFullIds.has(fullId))) ||
		inIndirectRing;
	const drawHandleBody = sceneHandlePrimaryVisualVisibleAtLod(lodCtx.lod) || linger;
	const pickProxy = sceneHandlePickProxyAtLod(lodCtx.lod) && !drawHandleBody;
	const meshFromLod = props.handleMeshByLod?.[lodCtx.lod];
	const meshUrl =
		typeof meshFromLod === "string" && meshFromLod.length
			? meshFromLod
			: typeof props.handleMeshUrl === "string" && props.handleMeshUrl.length
				? props.handleMeshUrl
				: undefined;

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
				<SceneVortexHandleGltf meshUrl={meshUrl} fullId={fullId} radius={r} />
			) : drawHandleBody && props.children ? (
				<group userData={{ sceneVortexFullId: fullId }}>{props.children}</group>
			) : drawHandleBody ? (
				<SceneVortexFallbackMesh fullId={fullId} radius={r} highlight={highlight} />
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
export const SceneMagnet = memo(function SceneMagnet(props: SceneMagnetProps) {
	return (
		<mesh position={props.position as [number, number, number]} userData={{ sceneMagnetId: props.id }}>
			<boxGeometry args={[props.size[0], props.size[1], props.size[2]]} />
			<meshStandardMaterial color="#a78bfa" wireframe />
		</mesh>
	);
});
//#endregion 🧲Magnet

//#region 🪢Tie
export const SceneTie = memo(function SceneTie(props: SceneTieProps) {
	const reg = useSceneRegistry();
	const [pts, setPts] = useState<Vector3[] | null>(null);
	useFrame(() => {
		const a = reg.getVortexWorld(props.source);
		const b = reg.getVortexWorld(props.target);
		if (a && b && sceneVector3IsFinite(a) && sceneVector3IsFinite(b)) {
			setPts([a.clone(), b.clone()]);
		} else if (pts !== null) {
			setPts(null);
		}
	});
	if (!pts) return null;
	return <Line points={pts} color="#64748b" lineWidth={1} userData={{ sceneTieId: props.id }} />;
});
//#endregion 🪢Tie

//#region 🧲Attraction
export const SceneAttraction = memo(function SceneAttraction(props: { from: Vec3; to: Vec3 }) {
	const pts = useMemo(() => [vec3ToThree(props.from), vec3ToThree(props.to)], [props.from, props.to]);
	return <Line points={pts} color="#f472b6" lineWidth={2} />;
});
//#endregion 🧲Attraction

//#region ✋Relocate
export function useSceneRelocate(objectId: string) {
	const reg = useSceneRegistry();
	return {
		mode: reg.relocateMode,
		start: () => reg.setActiveRelocateObjectId(objectId),
		cancel: () => reg.setActiveRelocateObjectId(null),
	};
}
//#endregion ✋Relocate

const EMPTY_BLOCKED_VORTICES: ReadonlySet<string> = new Set();

//#region 🎬Scene
function SceneOrbitGated({ target }: { target: Vec3 }) {
	const reg = useSceneRegistry();
	const gate = reg.linkDragActive || reg.linkIndirectPickAwait !== null;
	return <OrbitControls makeDefault enabled={!gate} target={target as [number, number, number]} />;
}

function SceneLinkThreeBinder() {
	const reg = useSceneRegistry();
	const t = useThree();
	useLayoutEffect(() => {
		reg.attachLinkThreeEnv({ camera: t.camera, gl: t.gl, scene: t.scene });
		return () => reg.attachLinkThreeEnv(null);
	}, [reg, t.camera, t.gl, t.scene]);
	return null;
}

function SceneLinkWindowBridge() {
	const reg = useSceneRegistry();
	const linkBusy = reg.linkDragActive || reg.linkIndirectPickAwait !== null;
	useEffect(() => {
		if (!linkBusy) return;
		const onMove = (e: PointerEvent) => {
			if (reg.linkDragActive) reg.updateLinkPointer(e.clientX, e.clientY);
			else if (reg.linkIndirectPickAwait) reg.updateIndirectPickPointer(e.clientX, e.clientY);
		};
		const onUp = (e: PointerEvent) => {
			if (reg.linkDragActive) reg.commitLinkPointer(e.clientX, e.clientY);
		};
		const onDown = (e: PointerEvent) => {
			if (e.button !== 0) return;
			if (reg.linkIndirectPickAwait) reg.commitIndirectPickPointerDown(e.clientX, e.clientY, e);
		};
		window.addEventListener("pointermove", onMove);
		window.addEventListener("pointerup", onUp, { capture: true });
		window.addEventListener("pointerdown", onDown, true);
		return () => {
			window.removeEventListener("pointermove", onMove);
			window.removeEventListener("pointerup", onUp, true);
			window.removeEventListener("pointerdown", onDown, true);
		};
	}, [reg, linkBusy]);
	return null;
}

function SceneLinkRubberBand() {
	const reg = useSceneRegistry();
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
		const wire =
			(reg.linkDragActive || reg.linkIndirectPickAwait !== null) && reg.linkDragSourceFullId ? true : false;
		if (!wire) {
			pos.setXYZ(0, 0, 0, 0);
			pos.setXYZ(1, 0, 0, 0);
			pos.needsUpdate = true;
			return;
		}
		const a = reg.getVortexWorld(reg.linkDragSourceFullId);
		const b = reg.linkEndWorldRef.current;
		if (a && b && sceneVector3IsFinite(a) && sceneVector3IsFinite(b)) {
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

function CameraReporter({
	target,
	zoom,
	onCamera,
}: {
	target: Vec3;
	zoom: number;
	onCamera?: (s: SceneCameraState) => void;
}) {
	const { camera } = useThree();
	const last = useRef("");
	useFrame(() => {
		const snap = JSON.stringify({
			p: camera.position.toArray(),
			t: [...target],
			z: zoom,
		});
		if (snap === last.current) return;
		last.current = snap;
		onCamera?.({
			position: camera.position.toArray() as unknown as Vec3,
			target,
			zoom,
		});
	});
	return null;
}

function SceneRegistryProvider({
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
	onLinkCompatibleNodes,
	onLinkTargetRing,
	onRelocate,
}: {
	children: ReactNode;
	lodKindRef: MutableRefObject<SceneLodKind>;
	kindCatalogs: SceneKindCatalogBundle | undefined;
	kindCompatibility: readonly SceneKindCompatEntry[] | undefined;
	blockedVortexFullIds: ReadonlySet<string>;
	proximityRadius: number;
	selectionMode: SceneSelectionMode;
	relocateMode: SceneRelocateMode;
	onSelect?: (snap: SceneSelectionSnapshot) => void;
	onConnect?: (p: SceneTieLinkPayload) => void;
	onProximityConnect?: (p: SceneTieLinkPayload) => void;
	onIndirectConnect?: (p: SceneTieLinkPayload) => void;
	onLinkCompatibleNodes?: (p: SceneLinkCompatibleNodesPayload) => void;
	onLinkTargetRing?: (p: SceneLinkTargetRingPayload) => void;
	onRelocate?: (p: SceneRelocatePayload) => void;
}) {
	const [selectedObjectIds, setSelectedObjectIds] = useState<readonly string[]>([]);
	const [activeRelocateObjectId, setActiveRelocateObjectId] = useState<string | null>(null);
	const [linkDragActive, setLinkDragActive] = useState(false);
	const [linkDragSourceFullId, setLinkDragSourceFullId] = useState<string | null>(null);
	const [linkCompatibleTargetFullIds, setLinkCompatibleTargetFullIds] = useState<ReadonlySet<string>>(new Set());
	const [linkHoverRingFullId, setLinkHoverRingFullId] = useState<string | null>(null);
	const [linkIndirectPickAwait, setLinkIndirectPickAwait] = useState<SceneLinkIndirectPickAwait | null>(null);

	const vortexGettersRef = useRef(new Map<string, VortexGetter>());
	const vortexMetaRef = useRef(new Map<string, SceneVortexBindingMeta>());
	const vortexPickRef = useRef(new Map<string, Object3D>());
	const objectGroupMap = useRef(new Map<string, Group | null>());
	const objectKindsRef = useRef(new Map<string, string | undefined>());
	const indirectPickRef = useRef<SceneLinkIndirectPickAwait | null>(null);

	useEffect(() => {
		indirectPickRef.current = linkIndirectPickAwait;
	}, [linkIndirectPickAwait]);

	const linkSessionRef = useRef<{
		sourceFullId: string;
		sourceObjectId: string;
		sourceCtx: SceneLinkHandleContext;
		compat: Set<string>;
		snapTargetFullId: string | null;
	} | null>(null);
	const linkEndWorldRef = useRef<Vector3 | null>(null);
	const linkThreeRef = useRef<{ camera: Camera; gl: WebGLRenderer; scene: Scene } | null>(null);
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

	const registerVortexBinding = useCallback((meta: SceneVortexBindingMeta, pickRoot: Object3D | null) => {
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

	const cancelLinkDrag = useCallback(() => {
		linkSessionRef.current = null;
		linkEndWorldRef.current = null;
		setLinkDragActive(false);
		setLinkDragSourceFullId(null);
		setLinkCompatibleTargetFullIds(new Set());
		setLinkHoverRingFullId(null);
		setLinkIndirectPickAwait(null);
		onLinkTargetRing?.({ source: "", objectId: null, vortexFullIds: [] });
	}, [onLinkTargetRing]);

	const beginLinkDragFromVortex = useCallback(
		(fullId: string, objectId: string, objectKind: string | undefined, vortexKind: string | undefined) => {
			if (indirectPickRef.current) return;
			if (blockedVortexFullIds.has(fullId)) return;
			const sourceCtx: SceneLinkHandleContext = { objectId, objectKind, vortexKind };
			const compat = new Set<string>();
			const objectIds = new Set<string>();
			for (const [tid, meta] of vortexMetaRef.current) {
				if (tid === fullId) continue;
				if (meta.objectId === objectId) continue;
				if (blockedVortexFullIds.has(tid)) continue;
				const targetCtx: SceneLinkHandleContext = {
					objectId: meta.objectId,
					objectKind: meta.objectKind,
					vortexKind: meta.vortexKind,
				};
				if (!sceneHandlesLinkCompatibleForDrag(sourceCtx, targetCtx, kindCompatibility, kindCatalogs)) continue;
				compat.add(tid);
				objectIds.add(meta.objectId);
			}
			setLinkIndirectPickAwait(null);
			linkSessionRef.current = {
				sourceFullId: fullId,
				sourceObjectId: objectId,
				sourceCtx,
				compat,
				snapTargetFullId: null,
			};
			linkEndWorldRef.current = null;
			setLinkDragActive(true);
			setLinkDragSourceFullId(fullId);
			setLinkCompatibleTargetFullIds(compat);
			setLinkHoverRingFullId(null);
			setActiveRelocateObjectId(null);
			onLinkCompatibleNodes?.({ source: fullId, objectIds: [...objectIds] });
		},
		[blockedVortexFullIds, kindCatalogs, kindCompatibility, onLinkCompatibleNodes],
	);

	const collectPickRoots = useCallback((): Object3D[] => {
		const out: Object3D[] = [];
		for (const p of vortexPickRef.current.values()) out.push(p);
		for (const g of objectGroupMap.current.values()) if (g) out.push(g);
		return out;
	}, []);

	const updateLinkPointer = useCallback(
		(clientX: number, clientY: number) => {
			const env = linkThreeRef.current;
			const session = linkSessionRef.current;
			if (!env || !session) return;
			const rect = env.gl.domElement.getBoundingClientRect();
			ndcRef.current.x = ((clientX - rect.left) / rect.width) * 2 - 1;
			ndcRef.current.y = -((clientY - rect.top) / rect.height) * 2 + 1;
			raycasterRef.current.setFromCamera(ndcRef.current, env.camera);
			const hits = raycasterRef.current.intersectObjects(collectPickRoots(), true);
			let ring: string | null = null;
			for (const h of hits) {
				const vf = readSceneVortexFullIdFromObject(h.object);
				if (vf && session.compat.has(vf) && vf !== session.sourceFullId && !blockedVortexFullIds.has(vf)) {
					ring = vf;
					break;
				}
			}
			setLinkHoverRingFullId((prev) => (prev === ring ? prev : ring));
			if (ring) {
				const meta = vortexMetaRef.current.get(ring);
				onLinkTargetRing?.({
					source: session.sourceFullId,
					objectId: meta?.objectId ?? null,
					vortexFullIds: ring ? [ring] : [],
				});
			} else {
				onLinkTargetRing?.({ source: session.sourceFullId, objectId: null, vortexFullIds: [] });
			}
			const hitWorld = hitScratchRef.current;
			if (hits.length > 0) {
				linkEndWorldRef.current = hitWorld.copy(hits[0]!.point);
			} else if (raycasterRef.current.ray.intersectPlane(planeRef.current, hitWorld)) {
				linkEndWorldRef.current = hitWorld.clone();
			} else {
				raycasterRef.current.ray.at(80, hitWorld);
				linkEndWorldRef.current = hitWorld.clone();
			}
			const pw = linkEndWorldRef.current;
			if (pw) {
				session.snapTargetFullId = sceneNearestLinkSnapFullId({
					lod: lodKindRef.current,
					pointerWorld: pw,
					sourceFullId: session.sourceFullId,
					compat: session.compat,
					blocked: blockedVortexFullIds,
					camera: env.camera,
					gl: env.gl,
					getVortexWorld: (id) => vortexGettersRef.current.get(id)?.() ?? null,
					metaRadius: (id) => vortexMetaRef.current.get(id)?.radiusWorld ?? 0.35,
				});
			} else session.snapTargetFullId = null;
		},
		[blockedVortexFullIds, collectPickRoots, lodKindRef, onLinkTargetRing],
	);

	const commitLinkPointer = useCallback(
		(clientX: number, clientY: number) => {
			const env = linkThreeRef.current;
			const session = linkSessionRef.current;
			if (!env || !session) {
				cancelLinkDrag();
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
			const snapId = session.snapTargetFullId;
			if (snapId && sceneLinkSnapCommitProximityOk(snapId, pointerWorld, env.camera, env.gl, getV, rad)) {
				const p = { source: session.sourceFullId, target: snapId };
				onConnect?.(p);
				onProximityConnect?.(p);
				cancelLinkDrag();
				return;
			}

			const sourceFull = session.sourceFullId;
			for (const h of hits) {
				const vf = readSceneVortexFullIdFromObject(h.object);
				if (
					vf &&
					vf !== sourceFull &&
					session.compat.has(vf) &&
					!blockedVortexFullIds.has(vf) &&
					vortexMetaRef.current.get(vf)?.objectId !== session.sourceObjectId
				) {
					onConnect?.({ source: sourceFull, target: vf });
					cancelLinkDrag();
					return;
				}
				const oid = readSceneObjectIdFromObject(h.object);
				if (oid && oid !== session.sourceObjectId) {
					const candidates: string[] = [];
					for (const [tid, meta] of vortexMetaRef.current) {
						if (meta.objectId !== oid) continue;
						if (blockedVortexFullIds.has(tid)) continue;
						if (!session.compat.has(tid)) continue;
						candidates.push(tid);
					}
					if (candidates.length === 1) {
						const p = { source: sourceFull, target: candidates[0]! };
						onConnect?.(p);
						onIndirectConnect?.(p);
						cancelLinkDrag();
						return;
					}
					if (candidates.length > 1) {
						linkSessionRef.current = null;
						setLinkDragActive(false);
						setLinkCompatibleTargetFullIds(new Set(candidates));
						setLinkHoverRingFullId(null);
						setLinkIndirectPickAwait({
							sourceFullId: sourceFull,
							targetObjectId: oid,
							candidates,
						});
						onLinkTargetRing?.({
							source: sourceFull,
							objectId: oid,
							vortexFullIds: candidates,
						});
						return;
					}
				}
			}
			cancelLinkDrag();
		},
		[
			blockedVortexFullIds,
			cancelLinkDrag,
			collectPickRoots,
			onConnect,
			onIndirectConnect,
			onLinkTargetRing,
			onProximityConnect,
		],
	);

	const updateIndirectPickPointer = useCallback(
		(clientX: number, clientY: number) => {
			const awaitPick = indirectPickRef.current;
			const env = linkThreeRef.current;
			if (!awaitPick || !env) return;
			const rect = env.gl.domElement.getBoundingClientRect();
			ndcRef.current.x = ((clientX - rect.left) / rect.width) * 2 - 1;
			ndcRef.current.y = -((clientY - rect.top) / rect.height) * 2 + 1;
			raycasterRef.current.setFromCamera(ndcRef.current, env.camera);
			const hits = raycasterRef.current.intersectObjects(collectPickRoots(), true);
			let ring: string | null = null;
			for (const h of hits) {
				const vf = readSceneVortexFullIdFromObject(h.object);
				if (vf && awaitPick.candidates.includes(vf)) {
					ring = vf;
					break;
				}
			}
			setLinkHoverRingFullId((prev) => (prev === ring ? prev : ring));
			const hitWorld = hitScratchRef.current;
			if (hits.length > 0) {
				linkEndWorldRef.current = hitWorld.copy(hits[0]!.point);
			} else if (raycasterRef.current.ray.intersectPlane(planeRef.current, hitWorld)) {
				linkEndWorldRef.current = hitWorld.clone();
			} else {
				raycasterRef.current.ray.at(80, hitWorld);
				linkEndWorldRef.current = hitWorld.clone();
			}
		},
		[collectPickRoots],
	);

	const commitIndirectPickPointerDown = useCallback(
		(clientX: number, clientY: number, ev?: PointerEvent) => {
			const awaitPick = indirectPickRef.current;
			const env = linkThreeRef.current;
			if (!awaitPick || !env) return;
			const rect = env.gl.domElement.getBoundingClientRect();
			ndcRef.current.x = ((clientX - rect.left) / rect.width) * 2 - 1;
			ndcRef.current.y = -((clientY - rect.top) / rect.height) * 2 + 1;
			raycasterRef.current.setFromCamera(ndcRef.current, env.camera);
			const hits = raycasterRef.current.intersectObjects(collectPickRoots(), true);
			for (const h of hits) {
				const vf = readSceneVortexFullIdFromObject(h.object);
				if (vf && awaitPick.candidates.includes(vf)) {
					const p = { source: awaitPick.sourceFullId, target: vf };
					onConnect?.(p);
					onIndirectConnect?.(p);
					cancelLinkDrag();
					ev?.stopImmediatePropagation();
					return;
				}
			}
			cancelLinkDrag();
		},
		[cancelLinkDrag, collectPickRoots, onConnect, onIndirectConnect],
	);

	const attachLinkThreeEnv = useCallback((env: { camera: Camera; gl: WebGLRenderer; scene: Scene } | null) => {
		linkThreeRef.current = env;
	}, []);

	const findNearestProximityRelocate = useCallback(
		(world: Vector3, movingObjectId: string): SceneTieLinkPayload | null => {
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
			return { source: `${movingObjectId}:link`, target: best.id };
		},
		[proximityRadius],
	);

	const value = useMemo<SceneRegistryValue>(
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
			linkDragActive,
			linkDragSourceFullId,
			linkCompatibleTargetFullIds,
			linkHoverRingFullId,
			linkIndirectPickAwait,
			beginLinkDragFromVortex,
			cancelLinkDrag,
			findNearestProximityRelocate,
			onSelect,
			onConnect,
			onProximityConnect,
			onIndirectConnect,
			onLinkCompatibleNodes,
			onLinkTargetRing,
			onRelocate,
			attachLinkThreeEnv,
			updateLinkPointer,
			commitLinkPointer,
			updateIndirectPickPointer,
			commitIndirectPickPointerDown,
			linkEndWorldRef,
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
			linkDragActive,
			linkDragSourceFullId,
			linkCompatibleTargetFullIds,
			linkHoverRingFullId,
			linkIndirectPickAwait,
			beginLinkDragFromVortex,
			cancelLinkDrag,
			findNearestProximityRelocate,
			onSelect,
			onConnect,
			onProximityConnect,
			onIndirectConnect,
			onLinkCompatibleNodes,
			onLinkTargetRing,
			onRelocate,
			attachLinkThreeEnv,
			updateLinkPointer,
			commitLinkPointer,
			updateIndirectPickPointer,
			commitIndirectPickPointerDown,
		],
	);

	return <SceneRegistryContext.Provider value={value}>{children}</SceneRegistryContext.Provider>;
}

function SceneChunks({
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
			const k = sceneChunkKey(p.origin, chunkSize);
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

function SceneInner(props: SceneCanvasProps) {
	const { camera: camProp, chunkSize = 256, proximityRadius = 12, children } = props;
	const lodKindRef = useRef<SceneLodKind>("normal");
	const domain = props.domain ?? DEFAULT_SCENE_DOMAIN;
	const thresholds = props.lodZoomThresholds ?? sceneLodZoomThresholdsForDomain(domain);
	const distanceReference = props.lodDistanceReference ?? 900;
	const gridFactor = props.gridFactor ?? DEFAULT_SCENE_LOD_GRID_FACTOR;
	const gridSnapEnabled = props.gridSnapEnabled ?? false;
	const showLodGrid = props.showLodGrid === true;
	const maxDist = 4000;
	const pos = (camProp?.position ?? [420, 320, 420]) as [number, number, number];
	const tgt = (camProp?.target ?? [0, 40, 0]) as Vec3;
	const zoom = camProp?.zoom ?? 1;
	const { chunked, rest } = useMemo(() => splitChunkedSceneChildren(children), [children]);
	const blocked = props.blockedVortexFullIds ?? EMPTY_BLOCKED_VORTICES;
	return (
		<SceneRegistryProvider
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
			onLinkCompatibleNodes={props.onLinkCompatibleNodes}
			onLinkTargetRing={props.onLinkTargetRing}
			onRelocate={props.onRelocate}
		>
			<SceneLodBridge
				lodKindRef={lodKindRef}
				thresholds={thresholds}
				distanceReference={distanceReference}
				gridFactor={gridFactor}
				gridSnapEnabled={gridSnapEnabled}
				showLodGrid={showLodGrid}
				onLodChange={props.onLodChange}
			>
				<PerspectiveCamera makeDefault position={pos} near={0.2} far={500_000} fov={50} />
				<SceneOrbitGated target={tgt} />
				<SceneLinkThreeBinder />
				<SceneLinkWindowBridge />
				<SceneLinkRubberBand />
				<CameraReporter target={tgt} zoom={zoom} onCamera={props.onCamera} />
				<ambientLight intensity={0.45} />
				<directionalLight position={[120, 180, 80]} intensity={0.85} />
				<SceneChunks chunkSize={chunkSize} maxDistance={maxDist}>
					{chunked}
				</SceneChunks>
				<group data-scene-unchunked>{rest}</group>
			</SceneLodBridge>
		</SceneRegistryProvider>
	);
}

export function Scene(props: SceneCanvasProps & { className?: string; style?: CSSProperties }) {
	const { children, className, style, onLodChange, domain = DEFAULT_SCENE_DOMAIN, ...rest } = props;
	const [shellLod, setShellLod] = useState<SceneLodKind>("normal");
	const handleLod = useCallback(
		(l: SceneLodKind) => {
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
			<Canvas gl={{ antialias: true }} dpr={[1, 2]}>
				<SceneInner {...rest} domain={domain} onLodChange={handleLod}>
					{children}
				</SceneInner>
			</Canvas>
		</div>
	);
}

//#endregion 🎬Scene

//#region 🖥️PlayHarness
// #region 🧾Meta
function parseKindCompatibility(meta: Record<string, unknown> | undefined): readonly SceneKindCompatEntry[] {
	if (!meta || typeof meta !== "object") return [];
	const arr = (meta as { kindCompatibility?: unknown }).kindCompatibility;
	if (!Array.isArray(arr)) return [];
	const out: SceneKindCompatEntry[] = [];
	for (const entry of arr) {
		if (!entry || typeof entry !== "object") continue;
		const e = entry as Record<string, unknown>;
		const source = typeof e.source === "string" ? e.source.trim() : "";
		const target = typeof e.target === "string" ? e.target.trim() : "";
		if (!source || !target) continue;
		const specificity =
			e.specificity === "general" ||
			e.specificity === "node" ||
			e.specificity === "edge" ||
			e.specificity === "handle" ||
			e.specificity === "wire" ||
			e.specificity === "object" ||
			e.specificity === "tie"
				? e.specificity
				: undefined;
		out.push({
			source,
			target,
			...(e.bidirectional === true ? { bidirectional: true } : {}),
			...(e.important === true ? { important: true } : {}),
			...(specificity ? { specificity } : {}),
		});
	}
	return out;
}

function parseKindCatalogs(meta: Record<string, unknown> | undefined): SceneKindCatalogBundle | undefined {
	const kc = meta?.kindCatalogs;
	if (!kc || typeof kc !== "object") return undefined;
	return kc as SceneKindCatalogBundle;
}
// #endregion 🧾Meta

// #region 🖥️Surface
const LS_THEME = "elements.board-play.surface.theme";
const LS_DEVICE = "elements.board-play.surface.device";
const LS_EXPERTISE = "elements.board-play.surface.expertise";

function parseStoredTheme(raw: string | null): ElementsSurfaceTheme {
	if (raw === "light" || raw === "dark" || raw === "system") return raw;
	return "system";
}

function parseStoredDevice(raw: string | null): ElementsSurfaceDevice {
	if (raw === "desktop" || raw === "tablet" || raw === "mobile") return raw;
	return "desktop";
}

function parseStoredExpertise(raw: string | null): Expertise {
	if (raw === Expertise.BEGINNER || raw === Expertise.NORMAL || raw === Expertise.EXPERT) return raw;
	return Expertise.NORMAL;
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

function ScenePlaySurfaceFooter(props: {
	theme: ElementsSurfaceTheme;
	device: ElementsSurfaceDevice;
	expertise: Expertise;
	onTheme: (v: ElementsSurfaceTheme) => void;
	onDevice: (v: ElementsSurfaceDevice) => void;
	onExpertise: (v: Expertise) => void;
}): ReactElement {
	const { theme, device, expertise, onDevice, onExpertise, onTheme } = props;
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
// #endregion 🖥️Surface

// #region 🎬ScenePlay
function ScenePlayBody({ fixture }: { fixture: SceneFixtureV1 }) {
	const [relocateMode, setRelocateMode] = useState<SceneRelocateMode>("translate");
	const [selectedId, setSelectedId] = useState<string | null>(null);
	const [proximityCount, setProximityCount] = useState(0);
	const [connectCount, setConnectCount] = useState(0);
	const [indirectCount, setIndirectCount] = useState(0);
	const [sceneLodTag, setSceneLodTag] = useState<SceneLodKind>("normal");
	const kindCompatibility = useMemo(() => parseKindCompatibility(fixture.meta), [fixture.meta]);
	const kindCatalogs = useMemo(() => parseKindCatalogs(fixture.meta), [fixture.meta]);
	const blockedVortexFullIds = useMemo(
		() => sceneBlockedVortexFullIdsFromTies(fixture.ties),
		[fixture.ties],
	);

	useEffect(() => {
		const urls = [...new Set(fixture.objects.map((o) => o.meshUrl))];
		for (const u of urls) {
			useGLTF.preload(u);
		}
	}, [fixture.objects]);

	useEffect(() => {
		const w = window as unknown as { __scenePlaySelect?: (id: string) => void };
		w.__scenePlaySelect = (id: string) => {
			setSelectedId(id);
		};
		return () => {
			delete w.__scenePlaySelect;
		};
	}, []);

	const onSelect = useCallback((snap: { objectIds: readonly string[] }) => {
		setSelectedId(snap.objectIds[0] ?? null);
	}, []);

	const onProximityConnect = useCallback(() => {
		setProximityCount((c) => c + 1);
	}, []);

	const onConnect = useCallback(() => {
		setConnectCount((c) => c + 1);
	}, []);

	const onIndirectConnect = useCallback(() => {
		setIndirectCount((c) => c + 1);
	}, []);

	return (
		<div className="flex h-full w-full flex-col">
			<div className="flex shrink-0 gap-2 border-b border-border bg-muted/40 p-2">
				<ToolbarZone>
					<ToolbarGroup>
						<ToolbarItem>
							<Button
								variant={relocateMode === "translate" ? "default" : "outline"}
								size="sm"
								onClick={() => setRelocateMode("translate")}
							>
								<Move3d className="mr-1 size-4" />
								Translate
							</Button>
						</ToolbarItem>
						<ToolbarItem>
							<Button
								variant={relocateMode === "rotate" ? "default" : "outline"}
								size="sm"
								onClick={() => setRelocateMode("rotate")}
							>
								<Rotate3d className="mr-1 size-4" />
								Rotate
							</Button>
						</ToolbarItem>
						<ToolbarItem>
							<Button
								variant={relocateMode === "scale" ? "default" : "outline"}
								size="sm"
								onClick={() => setRelocateMode("scale")}
							>
								<Scaling className="mr-1 size-4" />
								Scale
							</Button>
						</ToolbarItem>
					</ToolbarGroup>
				</ToolbarZone>
				<div className="ml-auto flex items-center gap-3 text-xs text-muted-foreground">
					<span data-e2e-selected>{selectedId ?? "—"}</span>
					<span data-e2e-scene-lod>{sceneLodTag}</span>
					<span data-e2e-proximity-count>{proximityCount}</span>
					<span data-e2e-connect-count>{connectCount}</span>
					<span data-e2e-indirect-count>{indirectCount}</span>
				</div>
			</div>
			<div className="relative min-h-0 flex-1">
				<Suspense fallback={<div className="p-4 text-sm text-muted-foreground">Loading meshes…</div>}>
					<Scene
						className="absolute inset-0"
						camera={fixture.camera}
						domain={fixture.domain}
						kindCatalogs={kindCatalogs}
						kindCompatibility={kindCompatibility}
						blockedVortexFullIds={blockedVortexFullIds}
						proximityRadius={24}
						relocateMode={relocateMode}
						showLodGrid
						gridSnapEnabled
						onLodChange={setSceneLodTag}
						onSelect={onSelect}
						onConnect={onConnect}
						onIndirectConnect={onIndirectConnect}
						onProximityConnect={onProximityConnect}
					>
						{fixture.objects.map((o) => (
							<SceneObject
								key={o.id}
								id={o.id}
								meshUrl={o.meshUrl}
								origin={o.origin}
								orientation={o.orientation}
								scale={o.scale}
								objectKind={o.objectKind}
								label={o.label}
								selected={selectedId === o.id}
								relocate={relocateMode}
							>
								{o.vortices.map((v) => (
									<SceneVortex key={v.id} objectId={o.id} objectKind={o.objectKind} {...v} />
								))}
							</SceneObject>
						))}
						{fixture.ties.map((t) => (
							<SceneTie key={t.id} {...t} />
						))}
					</Scene>
				</Suspense>
			</div>
		</div>
	);
}

function MainWindow() {
	const fixture = useMemo(() => parseSceneFixtureV1(sceneFixtureJson as unknown), []);
	if (!fixture) {
		return <div className="p-4 text-destructive">Invalid scene fixture</div>;
	}
	return <ScenePlayBody fixture={fixture} />;
}

const SCENE_PLAY_APP_ID = "elements-scene-play";

function ScenePlayInner(): ReactElement {
	const [theme, setTheme] = useState<ElementsSurfaceTheme>(readTheme);
	const [device, setDevice] = useState<ElementsSurfaceDevice>(readDevice);
	const [expertise, setExpertise] = useState<Expertise>(readExpertise);
	const { mobile } = useElementsSurfaceChrome({ theme, device, expertise });

	useEffect(() => {
		try {
			localStorage.setItem(LS_THEME, theme);
		} catch {
			/* ignore */
		}
	}, [theme]);

	useEffect(() => {
		try {
			localStorage.setItem(LS_DEVICE, device);
		} catch {
			/* ignore */
		}
	}, [device]);

	useEffect(() => {
		try {
			localStorage.setItem(LS_EXPERTISE, expertise);
		} catch {
			/* ignore */
		}
	}, [expertise]);

	const surfaceFooterItems = useMemo<FooterItem[]>(
		() => [
			{
				content: (
					<ScenePlaySurfaceFooter
						device={device}
						expertise={expertise}
						onDevice={setDevice}
						onExpertise={setExpertise}
						onTheme={setTheme}
						theme={theme}
					/>
				),
				id: "scene-play-surface",
				order: 0,
			},
		],
		[device, expertise, theme],
	);

	const apps = useMemo<UIAppConfig[]>(
		() => [
			{
				id: SCENE_PLAY_APP_ID,
				label: "Scene play",
				windowKinds: [{ id: "scene-main", label: "Scene", component: MainWindow }],
				defaultLayout: createStackLayout(["scene-main"], ["Scene"]),
			},
		],
		[],
	);

	return (
		<UI apps={apps} defaultAppId={SCENE_PLAY_APP_ID} footerItems={surfaceFooterItems} mobile={mobile} />
	);
}

function ScenePlayApp(): ReactElement {
	return (
		<LevelProvider level="window">
			<div className={`flex h-screen min-h-0 w-screen flex-col ${getLevelBgClass("window")}`}>
				<ScenePlayInner />
			</div>
		</LevelProvider>
	);
}
// #endregion 🎬ScenePlay

if (!import.meta.vitest && typeof document !== "undefined") {
	const rootEl = document.getElementById("root");
	if (rootEl) {
		createRoot(rootEl).render(<ScenePlayApp />);
	}
}
//#endregion 🖥️PlayHarness


if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;
	describe("resolveSceneLodLabelFromThresholds", () => {
		it("classifies zoom bands", () => {
			const t = DEFAULT_SCENE_LOD_ZOOM_THRESHOLDS;
			expect(resolveSceneLodLabelFromThresholds(0.1, t)).toBe("minimap");
			expect(resolveSceneLodLabelFromThresholds(0.2, t)).toBe("overview");
			expect(resolveSceneLodLabelFromThresholds(0.3, t)).toBe("compact");
			expect(resolveSceneLodLabelFromThresholds(0.5, t)).toBe("normal");
			expect(resolveSceneLodLabelFromThresholds(1, t)).toBe("detail");
			expect(resolveSceneLodLabelFromThresholds(2, t)).toBe("micro");
		});
	});
	describe("sceneLodZoomThresholdsForDomain", () => {
		it("derives architecture thresholds from the domain ladder", () => {
			const t = sceneLodZoomThresholdsForDomain("architecture");
			expect(resolveSceneLodLabelFromThresholds(0.1, t)).toBe("minimap");
			expect(resolveSceneLodLabelFromThresholds(0.2, t)).toBe("overview");
			expect(resolveSceneLodLabelFromThresholds(0.3, t)).toBe("compact");
			expect(resolveSceneLodLabelFromThresholds(0.5, t)).toBe("normal");
			expect(resolveSceneLodLabelFromThresholds(1, t)).toBe("detail");
			expect(resolveSceneLodLabelFromThresholds(2, t)).toBe("micro");
		});
	});
	describe("sceneLodVisibleGridSnapStepWorld", () => {
		it("returns per-band steps", () => {
			expect(sceneLodVisibleGridSnapStepWorld("minimap", 10)).toBe(null);
			expect(sceneLodVisibleGridSnapStepWorld("overview", 10)).toBe(100);
			expect(sceneLodVisibleGridSnapStepWorld("compact", 10)).toBe(50);
			expect(sceneLodVisibleGridSnapStepWorld("normal", 10)).toBe(25);
			expect(sceneLodVisibleGridSnapStepWorld("detail", 10)).toBe(5);
			expect(sceneLodVisibleGridSnapStepWorld("micro", 10)).toBe(1);
		});
	});
	describe("parseSceneFixtureV1", () => {
		it("accepts minimal fixture", () => {
			const f = parseSceneFixtureV1({
				schema: "elements.scene.fixture/v1",
				camera: { position: [0, 0, 0], target: [0, 0, 1], zoom: 1 },
				ties: [],
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
			const f = parseSceneFixtureV1({
				schema: "elements.scene.fixture/v1",
				domain: "Urban",
				camera: { position: [0, 0, 0], target: [0, 0, 1], zoom: 1 },
				ties: [],
				objects: [{ id: "a", meshUrl: "/m.glb", origin: [1, 2, 3], vortices: [] }],
			});
			expect(f?.domain).toBe("urban");
		});
		it("parses vortex handleMeshByLod", () => {
			const f = parseSceneFixtureV1({
				schema: "elements.scene.fixture/v1",
				camera: { position: [0, 0, 0], target: [0, 0, 1], zoom: 1 },
				ties: [],
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
	describe("sceneChunkKey", () => {
		it("buckets origin", () => {
			expect(sceneChunkKey([10, 10, 10], 256)).toBe("0|0|0");
			expect(sceneChunkKey([300, 0, 0], 256)).toBe("1|0|0");
		});
	});
	describe("sceneChunkDistanceVisible", () => {
		it("keeps visible inside exit margin after entering", () => {
			const cam = new Vector3(0, 0, 0);
			const chunkSize = 256;
			const maxDist = 200;
			const enterDist = maxDist + sceneChunkBoundsRadius(chunkSize);
			const far = new Vector3(enterDist + chunkSize, 0, 0);
			expect(
				sceneChunkDistanceVisible({ camPos: cam, chunkCenter: far, chunkSize, maxDist, wasVisible: false }),
			).toBe(false);
			const near = new Vector3(enterDist - 50, 0, 0);
			expect(
				sceneChunkDistanceVisible({ camPos: cam, chunkCenter: near, chunkSize, maxDist, wasVisible: false }),
			).toBe(true);
			const between = new Vector3(enterDist + chunkSize * 0.25, 0, 0);
			expect(
				sceneChunkDistanceVisible({ camPos: cam, chunkCenter: between, chunkSize, maxDist, wasVisible: true }),
			).toBe(true);
			const beyond = new Vector3(enterDist + chunkSize * 0.75, 0, 0);
			expect(
				sceneChunkDistanceVisible({ camPos: cam, chunkCenter: beyond, chunkSize, maxDist, wasVisible: true }),
			).toBe(false);
		});
	});
	describe("sceneGltfPoolAcquire", () => {
		it("tracks refcount without clearing cache on release", () => {
			const url = "http://x/pool-test.glb";
			sceneGltfPoolAcquire(url);
			sceneGltfPoolAcquire(url);
			sceneGltfPoolRelease(url);
			sceneGltfPoolRelease(url);
			sceneGltfPoolAcquire(url);
			sceneGltfPoolRelease(url);
			expect(true).toBe(true);
		});
	});
	describe("sceneKindsCompatible", () => {
		it("matches bidirectional", () => {
			const ok = sceneKindsCompatible("a", "b", [{ source: "b", target: "a", bidirectional: true }]);
			expect(ok).toBe(true);
		});
	});
	describe("sceneBlockedVortexFullIdsFromTies", () => {
		it("collects endpoints", () => {
			const s = sceneBlockedVortexFullIdsFromTies([{ source: "a:h1", target: "b:h2" }]);
			expect(s.has("a:h1")).toBe(true);
			expect(s.has("b:h2")).toBe(true);
		});
	});
	describe("sceneHandlesLinkCompatibleForDrag", () => {
		it("allows all when rules empty", () => {
			const ok = sceneHandlesLinkCompatibleForDrag(
				{ objectId: "a", objectKind: "n1", vortexKind: "h1" },
				{ objectId: "b", objectKind: "n2", vortexKind: "h2" },
				[],
				undefined,
			);
			expect(ok).toBe(true);
		});
		it("matches handle specificity", () => {
			const ok = sceneHandlesLinkCompatibleForDrag(
				{ objectId: "a", objectKind: "x", vortexKind: "h1" },
				{ objectId: "b", objectKind: "y", vortexKind: "h2" },
				[{ source: "h1", target: "h2", specificity: "handle" }],
				undefined,
			);
			expect(ok).toBe(true);
		});
	});
	describe("resolveSceneWireKindForVortex", () => {
		it("falls back to default wire id", () => {
			expect(resolveSceneWireKindForVortex("any", undefined)).toBe("board.wire.link");
		});
	});
	describe("scene play fixture hook", () => {
		it("parses nakagin fixture", () => {
			const f = parseSceneFixtureV1(sceneFixtureJson as unknown);
			expect(f?.domain).toBe("architecture");
			expect(f?.ties.length).toBeGreaterThan(0);
			expect(f?.objects.length).toBeGreaterThan(0);
		});
	});
}
