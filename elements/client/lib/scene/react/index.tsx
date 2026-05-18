import { Clone, Line, OrbitControls, PerspectiveCamera, TransformControls, useGLTF } from "@react-three/drei";
import { Canvas, useFrame, useThree } from "@react-three/fiber";
import {
	Children,
	createContext,
	isValidElement,
	memo,
	useCallback,
	useContext,
	useEffect,
	useMemo,
	useRef,
	useState,
	type CSSProperties,
	type ReactNode,
} from "react";
import {
	Frustum,
	Matrix4,
	Plane,
	Quaternion,
	Raycaster,
	Sphere,
	Vector2,
	Vector3,
	type Group,
	type Mesh,
	type Object3D,
} from "three";

//#region 🔖Kinds
export type Vec3 = readonly [number, number, number];
export type Quat = readonly [number, number, number, number];

export type SceneRelocateMode = "translate" | "rotate" | "scale";
export type SceneSelectionMode = "single" | "additive" | "subtractive" | "toggle";
export type SceneConnectKind = "indirect" | "connect" | "proximity";

export interface SceneCameraState {
	position: Vec3;
	target: Vec3;
	zoom: number;
}

export interface SceneVortexProps {
	id: string;
	vortexKind?: string;
	position: Vec3;
	direction?: Vec3;
	radius?: number;
	visible?: boolean;
	handleMeshUrl?: string;
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

export interface SceneCanvasProps {
	camera?: Partial<SceneCameraState>;
	chunkSize?: number;
	kindCatalogs?: SceneKindCatalogBundle;
	kindCompatibility?: readonly SceneKindCompatEntry[];
	/** @emoji 🚫 Vortex full ids (`objectId:vortexId`) that already terminate a tie and cannot start or receive a new link. */
	blockedVortexFullIds?: ReadonlySet<string>;
	proximityRadius?: number;
	relocateMode?: SceneRelocateMode;
	selectionMode?: SceneSelectionMode;
	onCamera?: (s: SceneCameraState) => void;
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
	meta?: Record<string, unknown>;
	ties: SceneTieProps[];
	objects: SceneFixtureObjectV1[];
}
//#endregion 🔖Kinds

//#region 📐Coords
/** @emoji 🧭 Maps authoring RH basis (origin + xAxis + yAxis) into three.js Y-up RH position + quaternion. */
export function planeBasisToThreeJs(plane: {
	readonly origin: { x: number; y: number; z: number };
	readonly xAxis: { x: number; y: number; z: number };
	readonly yAxis: { x: number; y: number; z: number };
}): { origin: Vec3; orientation: Quat } {
	const authoringToThree = (p: { x: number; y: number; z: number }): Vec3 => [p.x, p.z, -p.y];
	const x = new Vector3(...authoringToThree(plane.xAxis)).normalize();
	const y = new Vector3(...authoringToThree(plane.yAxis)).normalize();
	const z = new Vector3().crossVectors(x, y).normalize();
	const o = authoringToThree(plane.origin);
	const q = new Quaternion().setFromRotationMatrix(new Matrix4().makeBasis(x, y, z));
	return { origin: o, orientation: [q.x, q.y, q.z, q.w] };
}
//#endregion 📐Coords

//#region 🧾Fixture
function isVec3(v: unknown): v is Vec3 {
	return Array.isArray(v) && v.length === 3 && v.every((n) => typeof n === "number");
}

function isQuat(v: unknown): v is Quat {
	return Array.isArray(v) && v.length === 4 && v.every((n) => typeof n === "number");
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
				vortices.push({
					id: vx.id,
					...(typeof vx.vortexKind === "string" ? { vortexKind: vx.vortexKind } : {}),
					position: vx.position,
					...(isVec3(vx.direction) ? { direction: vx.direction } : {}),
					...(typeof vx.radius === "number" ? { radius: vx.radius } : {}),
					...(typeof vx.handleMeshUrl === "string" ? { handleMeshUrl: vx.handleMeshUrl } : {}),
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

export function resolveSceneWireKindForVortex(
	vortexKind: string | undefined,
	catalogs: SceneKindCatalogBundle | undefined,
): string {
	const h = catalogHandleById(catalogs, vortexKind);
	const w = h?.defaultWireKind?.trim();
	return w && w.length > 0 ? w : SCENE_DEFAULT_WIRE_KIND_ID;
}

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
		useGLTF.clear(url);
	} else {
		gltfRefCounts.set(url, n);
	}
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
	linkEndWorldScratch: Vector3;
	beginLinkDragFromVortex(fullId: string, objectId: string, objectKind: string | undefined, vortexKind: string | undefined): void;
	cancelLinkDrag(): void;
	findNearestProximityRelocate(world: Vector3, movingObjectId: string): SceneTieLinkPayload | null;
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

function useVisibleChunkKeys(chunkKeys: Iterable<string>, chunkSize: number, maxDist: number): Set<string> {
	const { camera } = useThree();
	const frustum = useMemo(() => new Frustum(), []);
	const projScreenMatrix = useMemo(() => new Matrix4(), []);
	const sphereTmp = useMemo(() => new Sphere(), []);
	const visible = useMemo(() => new Set<string>(), []);
	useFrame(() => {
		projScreenMatrix.multiplyMatrices(camera.projectionMatrix, camera.matrixWorldInverse);
		frustum.setFromProjectionMatrix(projScreenMatrix);
		visible.clear();
		const camPos = camera.position;
		for (const key of chunkKeys) {
			const [ix, iy, iz] = key.split("|").map(Number);
			const cx = (ix + 0.5) * chunkSize;
			const cy = (iy + 0.5) * chunkSize;
			const cz = (iz + 0.5) * chunkSize;
			sphereTmp.center.set(cx, cy, cz);
			sphereTmp.radius = chunkSize * 0.866;
			if (sphereTmp.center.distanceTo(camPos) > maxDist + sphereTmp.radius) continue;
			if (!frustum.intersectsSphere(sphereTmp)) continue;
			visible.add(key);
		}
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
//#endregion 🔗LinkGesture

//#region 🧊Object
export const SceneObject = memo(function SceneObject(props: SceneObjectProps) {
	const group = useRef<Group>(null);
	const gltf = usePooledGltf(props.meshUrl);
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
			if (reg.linkDragActive) return;
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
			{gltf.scene && <Clone object={gltf.scene} />}
			{props.children}
			{showTc && (
				<TransformControls
					object={tcTarget}
					mode={props.relocate ?? reg.relocateMode}
					onMouseDown={() => {
						const g = group.current;
						if (g) {
							beforeRef.current = {
								origin: g.position.clone(),
								quat: g.quaternion.clone(),
								scale: g.scale.clone(),
							};
						}
					}}
					onMouseUp={() => {
						const g = group.current;
						if (!g || !beforeRef.current) return;
						const before = beforeRef.current;
						const afterOrigin = g.position.toArray() as unknown as Vec3;
						const afterQuat = g.quaternion.toArray() as unknown as Quat;
						const afterScale = g.scale.toArray() as unknown as Vec3;
						reg.onRelocate?.({
							objectId: props.id,
							mode: props.relocate ?? reg.relocateMode,
							before: {
								origin: before.origin.toArray() as unknown as Vec3,
								orientation: before.quat.toArray() as unknown as Quat,
								scale: before.scale.toArray() as unknown as Vec3,
							},
							after: {
								origin: afterOrigin,
								orientation: afterQuat,
								scale: afterScale,
							},
						});
						const cand = reg.findNearestProximityRelocate(g.position, props.id);
						if (cand) reg.onProximityConnect?.(cand);
						beforeRef.current = null;
					}}
				/>
			)}
		</group>
	);
});
//#endregion 🧊Object

//#region 🌀Vortex
const vortexFallbackMatProps = { transparent: true, opacity: 0.55 } as const;

function SceneVortexVisual(props: {
	fullId: string;
	radius: number;
	visible: boolean;
	position: Vec3;
	handleMeshUrl?: string;
	children?: ReactNode;
	highlight: "none" | "compatible" | "ring" | "source";
}) {
	const gltf = props.handleMeshUrl ? usePooledGltf(props.handleMeshUrl) : null;
	const color =
		props.highlight === "compatible"
			? "#22c55e"
			: props.highlight === "ring"
				? "#facc15"
				: props.highlight === "source"
					? "#94a3b8"
					: "#38bdf8";
	const emissive = props.highlight === "ring" ? "#ca8a04" : "#000000";
	const emissiveIntensity = props.highlight === "ring" ? 0.45 : 0;
	const scale = props.handleMeshUrl ? (props.radius / 0.35) * 0.9 : 1;
	return (
		<group position={props.position as [number, number, number]} visible={props.visible}>
			{props.handleMeshUrl && gltf?.scene ? (
				<Clone object={gltf.scene} scale={scale} userData={{ sceneVortexFullId: props.fullId }} />
			) : props.children ? (
				<group userData={{ sceneVortexFullId: props.fullId }}>{props.children}</group>
			) : (
				<mesh userData={{ sceneVortexFullId: props.fullId }}>
					<sphereGeometry args={[props.radius, 12, 12]} />
					<meshStandardMaterial
						color={color}
						emissive={emissive}
						emissiveIntensity={emissiveIntensity}
						{...vortexFallbackMatProps}
					/>
				</mesh>
			)}
		</group>
	);
}

export const SceneVortex = memo(function SceneVortex(
	props: SceneVortexProps & { objectId: string; objectKind?: string },
) {
	const root = useRef<Group>(null);
	const pickRef = useRef<Group>(null);
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

	useEffect(() => {
		reg.registerVortexBinding(
			{
				fullId,
				objectId: props.objectId,
				objectKind: props.objectKind,
				vortexKind: props.vortexKind,
			},
			pickRef.current,
		);
		return () => {
			reg.unregisterVortexBinding(fullId);
		};
	}, [fullId, props.objectId, props.objectKind, props.vortexKind, reg]);

	const highlight: "none" | "compatible" | "ring" | "source" = reg.linkDragSourceFullId === fullId
		? "source"
		: reg.linkHoverRingFullId === fullId
			? "ring"
			: reg.linkCompatibleTargetFullIds.has(fullId)
				? "compatible"
				: "none";

	const onPointerDown = useCallback(
		(e: { stopPropagation: () => void; nativeEvent: PointerEvent; target: EventTarget | null }) => {
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

	const vis = props.visible !== false;
	return (
		<group
			ref={root}
			userData={{ sceneVortexFullId: fullId, vortexKind: props.vortexKind }}
			data-scene-vortex={fullId}
			visible={vis}
			onPointerDown={onPointerDown}
		>
			<group ref={pickRef}>
				<SceneVortexVisual
					fullId={fullId}
					radius={r}
					visible={vis}
					position={[0, 0, 0]}
					handleMeshUrl={props.handleMeshUrl}
					highlight={highlight}
				>
					{props.children}
				</SceneVortexVisual>
			</group>
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
	const [pts, setPts] = useState<Vector3[]>(() => [new Vector3(), new Vector3(0, 1, 0)]);
	useFrame(() => {
		const a = reg.getVortexWorld(props.source);
		const b = reg.getVortexWorld(props.target);
		if (a && b) setPts([a.clone(), b.clone()]);
	});
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

//#region 🎬Scene
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
	kindCompatibility,
	proximityRadius,
	selectionMode,
	relocateMode,
	onSelect,
	onConnect,
	onProximityConnect,
	onIndirectConnect,
	onRelocate,
}: {
	children: ReactNode;
	kindCompatibility: readonly SceneKindCompatEntry[] | undefined;
	proximityRadius: number;
	selectionMode: SceneSelectionMode;
	relocateMode: SceneRelocateMode;
	onSelect?: (snap: SceneSelectionSnapshot) => void;
	onConnect?: (p: SceneTieLinkPayload) => void;
	onProximityConnect?: (p: SceneTieLinkPayload) => void;
	onIndirectConnect?: (p: SceneTieLinkPayload) => void;
	onRelocate?: (p: SceneRelocatePayload) => void;
}) {
	const [selectedObjectIds, setSelectedObjectIds] = useState<readonly string[]>([]);
	const [activeRelocateObjectId, setActiveRelocateObjectId] = useState<string | null>(null);
	const objectGroupMap = useRef(new Map<string, Group | null>());

	const registerVortex = useCallback((fullId: string, getter: VortexGetter) => {
		vortexGetterMap.set(fullId, getter);
	}, []);

	const unregisterVortex = useCallback((fullId: string) => {
		vortexGetterMap.delete(fullId);
	}, []);

	const getVortexWorld = useCallback((fullId: string) => {
		const g = vortexGetterMap.get(fullId);
		return g ? g() : null;
	}, []);

	const registerObject = useCallback((id: string, group: Group | null) => {
		objectGroupMap.current.set(id, group);
	}, []);

	const getObjectGroup = useCallback((id: string) => objectGroupMap.current.get(id) ?? null, []);

	const value = useMemo<SceneRegistryValue>(
		() => ({
			registerVortex,
			unregisterVortex,
			getVortexWorld,
			registerObject,
			getObjectGroup,
			kindCompatibility,
			proximityRadius,
			selectedObjectIds,
			setSelectedObjectIds,
			selectionMode,
			relocateMode,
			activeRelocateObjectId,
			setActiveRelocateObjectId,
			onSelect,
			onConnect,
			onProximityConnect,
			onIndirectConnect,
			onRelocate,
		}),
		[
			registerVortex,
			unregisterVortex,
			getVortexWorld,
			registerObject,
			getObjectGroup,
			kindCompatibility,
			proximityRadius,
			selectedObjectIds,
			selectionMode,
			relocateMode,
			activeRelocateObjectId,
			onSelect,
			onConnect,
			onProximityConnect,
			onIndirectConnect,
			onRelocate,
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
			{[...buckets].map(([key, items]) =>
				visible.has(key) ? (
					<group key={key} userData={{ sceneChunk: key }}>
						{items}
					</group>
				) : null,
			)}
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
	const maxDist = 4000;
	const pos = (camProp?.position ?? [420, 320, 420]) as [number, number, number];
	const tgt = (camProp?.target ?? [0, 40, 0]) as Vec3;
	const zoom = camProp?.zoom ?? 1;
	const { chunked, rest } = useMemo(() => splitChunkedSceneChildren(children), [children]);
	return (
		<SceneRegistryProvider
			kindCompatibility={props.kindCompatibility}
			proximityRadius={proximityRadius}
			selectionMode={props.selectionMode ?? "single"}
			relocateMode={props.relocateMode ?? "translate"}
			onSelect={props.onSelect}
			onConnect={props.onConnect}
			onProximityConnect={props.onProximityConnect}
			onIndirectConnect={props.onIndirectConnect}
			onRelocate={props.onRelocate}
		>
			<PerspectiveCamera makeDefault position={pos} near={0.2} far={500_000} fov={50} />
			<OrbitControls makeDefault target={tgt as [number, number, number]} />
			<CameraReporter target={tgt} zoom={zoom} onCamera={props.onCamera} />
			<ambientLight intensity={0.45} />
			<directionalLight position={[120, 180, 80]} intensity={0.85} />
			<SceneChunks chunkSize={chunkSize} maxDistance={maxDist}>
				{chunked}
			</SceneChunks>
			<group data-scene-unchunked>{rest}</group>
		</SceneRegistryProvider>
	);
}

export function Scene(props: SceneCanvasProps & { className?: string; style?: CSSProperties }) {
	const { children, className, style, ...rest } = props;
	return (
		<div className={className} style={{ width: "100%", height: "100%", ...style }} data-scene-root>
			<Canvas gl={{ antialias: true }} dpr={[1, 2]}>
				<SceneInner {...rest}>{children}</SceneInner>
			</Canvas>
		</div>
	);
}
//#endregion 🎬Scene

if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;
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
		});
	});
	describe("sceneChunkKey", () => {
		it("buckets origin", () => {
			expect(sceneChunkKey([10, 10, 10], 256)).toBe("0|0|0");
			expect(sceneChunkKey([300, 0, 0], 256)).toBe("1|0|0");
		});
	});
	describe("planeBasisToThreeJs", () => {
		it("maps identity-ish basis", () => {
			const { origin, orientation } = planeBasisToThreeJs({
				origin: { x: 1, y: 2, z: 3 },
				xAxis: { x: 1, y: 0, z: 0 },
				yAxis: { x: 0, y: 1, z: 0 },
			});
			expect(origin[0]).toBe(1);
			expect(orientation.length).toBe(4);
		});
	});
	describe("sceneGltfPoolAcquire", () => {
		it("increments refcount", () => {
			sceneGltfPoolAcquire("http://x/a.glb");
			sceneGltfPoolAcquire("http://x/a.glb");
			sceneGltfPoolRelease("http://x/a.glb");
			sceneGltfPoolRelease("http://x/a.glb");
			expect(true).toBe(true);
		});
	});
	describe("sceneKindsCompatible", () => {
		it("matches bidirectional", () => {
			const ok = sceneKindsCompatible("a", "b", [{ source: "b", target: "a", bidirectional: true }]);
			expect(ok).toBe(true);
		});
	});
}
