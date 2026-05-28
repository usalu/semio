/// <reference types="vite/client" />
/// <reference types="vitest/importMeta" />
// #region 🧲Header
/** @emoji 🎬 `@spatial/js-renderer-r3f` — R3F factory renderer with {@link InteractionRepl} host props/`on*` callbacks, {@link InteractionCanvas}, and {@link InteractionSpatialView}. See `spatial/assets/modelDefinition/geometry/typology/primitive/box/interaction/box.json`. */
// #endregion 🧲Header

// #region 📥Imports
import { Line, OrbitControls, Text } from "@react-three/drei";
import { Canvas, useFrame, useThree, type ThreeEvent } from "@react-three/fiber";
import {
	Suspense,
	useCallback,
	useEffect,
	useMemo,
	useRef,
	useState,
	useSyncExternalStore,
	type CSSProperties,
	type KeyboardEvent,
	type ReactNode,
} from "react";
import { MOUSE } from "three";
import * as THREE from "three";

THREE.Object3D.DEFAULT_UP.set(0, 0, 1);

import {
	abortActiveInteractionSession,
	applyModelDiff,
	solidRef,
	createInteractionRuntime,
	emptyMeshTransfer,
	DocumentHistory,
	EMPTY_MODEL_DIFF,
	interactionCanConfirmSelection,
	InteractionRegistry,
	isEmptyModelDiff,
	isInteractionSessionActive,
	isGeometryModelDefinition,
	GEOMETRY_MODEL_DEFINITION_ID,
	listModelDefinitionManifests,
	listTransformationsFromModelDefinition,
	listTransformationsIntoModelDefinition,
	listSpatialInteractionsForModelDefinition,
	resolveSpatialInteractionKeyForModelDefinition,
	modelDefinitionSelectionEntityKinds,
	modelDefinitionUsesGeometryPicking,
	countViewObjectsForModelDefinition,
	primaryAttributeSelectionTarget,
	listAttributeDefinitionsForModelDefinitionEntity,
	attributeDefinitionEditorKind,
	attributeDefinitionValueOptions,
	derivePropertyValue,
	listApplicablePropertyDefinitionsForModelDefinition,
	listTypologiesForModelDefinition,
	validateAttributeValue,
	resolveModelDefinitionScope,
	type AttributeDefinitionSpec,
	applyTransformation,
	qualifiedTransformationId,
	selectionOperationUsesModelObjects,
	selectionSeedTargetsForOperation,
	loadSpatialInteraction,
	parseModelJson,
	listKeyedInteractionTransitions,
	Model,
	type InteractionEvent,
	type InteractionKeybindRow,
	type InteractionRuntime,
	type InteractionRuntimeOptions,
	type InteractionSnapshot,
	type InteractionSpec,
	type DisplayItem,
	type DisplayModel,
	type TransformationSpec,
	kernelGeometry,
	type SpatialKernel,
	type SpatialPreviewKernel,
	type ModelDocument,
	type SelectionTarget,
	type SpatialObjectRecord,
	type ShellRecord,
	type FaceGroup,
	type FaceInfo,
	type MeshTransfer,
	type SpatialInteraction,
	type ModelEntityKind,
	type ModelJson,
	type ObjectRef,
	type Vec3,
	type SpatialComputeMode,
} from "@spatial/js-core";

type AnchorRecord = kernelGeometry.AnchorRecord;
type AnchorRef = kernelGeometry.AnchorRef;
type VertexRef = kernelGeometry.VertexRef;
type ShellRef = kernelGeometry.ShellRef;
type SolidRef = kernelGeometry.SolidRef;
type SolidRecord = kernelGeometry.SolidRecord;
type EdgeRecord = kernelGeometry.EdgeRecord;
type FaceRecord = kernelGeometry.FaceRecord;
type ShellRecord = kernelGeometry.ShellRecord;
type VertexRecord = kernelGeometry.VertexRecord;
type WireRecord = kernelGeometry.WireRecord;

export type { SpatialComputeMode };
import { PreciseSpatialKernelMath, preciseSpatialKernelMath } from "@spatial/js-kernel-brepjs";
// #endregion 📥Imports

// #region ⚡R3FPreviewKernel
/** @emoji ⚡ Fast approximate `SpatialPreviewKernel` for live R3F previews (lower tessellation). */
export class R3FPreviewKernel extends PreciseSpatialKernelMath {
	override arcSamplePoints = (center: Vec3, start: Vec3, end: Vec3, segments = 12): readonly Vec3[] =>
		preciseSpatialKernelMath.arcSamplePoints(center, start, end, segments);

	override edgeSamplePoints = (
		vertices: Readonly<Record<string, VertexRecord>>,
		edge: EdgeRecord,
		segments = 12,
	): readonly Vec3[] => preciseSpatialKernelMath.edgeSamplePoints(vertices, edge, segments);

	override circleSamplePoints = (center: Vec3, normal: Vec3, radius: number, segments = 24): readonly Vec3[] =>
		preciseSpatialKernelMath.circleSamplePoints(center, normal, radius, segments);

	override nurbsDisplaySamplePoints = (poles: readonly Vec3[], segmentsPerSpan = 6): readonly Vec3[] =>
		preciseSpatialKernelMath.nurbsDisplaySamplePoints(poles, segmentsPerSpan);
}

/** @emoji ⚡ Default fast preview kernel for play and R3F hosts. */
export const r3fPreviewKernel = new R3FPreviewKernel();

const scenePreviewKernelRef: { current: SpatialPreviewKernel } = { current: r3fPreviewKernel };

/** @emoji ⚡ Binds the active scene preview kernel (fast vs precise) for R3F wireframe helpers. */
export function bindScenePreviewKernel(kernel: SpatialPreviewKernel): void {
	scenePreviewKernelRef.current = kernel;
}

function scenePreview(): SpatialPreviewKernel {
	return scenePreviewKernelRef.current;
}
// #endregion ⚡R3FPreviewKernel

// #region 🎬WorkerClient
/** @emoji 🧩 Binary search `faceGroups` by triangle index (playground `ShapeRenderer` pattern). */
export function findFaceGroupAt(groups: readonly FaceGroup[], triangleIndex: number): FaceGroup | null {
	const indexBufferOffset = triangleIndex * 3;
	let lo = 0;
	let hi = groups.length - 1;
	while (lo <= hi) {
		const mid = (lo + hi) >>> 1;
		const group = groups[mid]!;
		if (indexBufferOffset < group.start) hi = mid - 1;
		else if (indexBufferOffset >= group.start + group.count) lo = mid + 1;
		else return group;
	}
	return null;
}

/** @emoji 🎞️ Debounced `SpatialKernel.tessellate` for R3F hosts (worker-backed brepjs). */
export function useTessellation(
	kernel: SpatialKernel | null,
	solid: ReturnType<typeof solidRef> | null,
	tolerance: number,
): MeshTransfer | null {
	const [mesh, setMesh] = useState<MeshTransfer | null>(null);
	const rafRef = useRef(0);
	useEffect(() => {
		if (!kernel || !solid) {
			setMesh(null);
			return;
		}
		cancelAnimationFrame(rafRef.current);
		rafRef.current = requestAnimationFrame(() => {
			void kernel.tessellate(solid, tolerance).then((next) => setMesh(isRenderableMeshTransfer(next) ? next : null));
		});
		return () => cancelAnimationFrame(rafRef.current);
	}, [kernel, solid, tolerance]);
	return mesh;
}

/** @emoji 📦 Lists `SolidRef` ids present on a model graph (document solids for tessellation). */
export function listModelSolidRefs(model: Model | ModelJson | null): readonly SolidRef[] {
	if (!model) return [];
	const graph = model instanceof Model ? model : parseModelJson(model);
	if (!graph) return [];
	return Object.keys(graph.solids).map((id) => solidRef(id));
}

/** @emoji 🔑 Stable React key from mesh buffer fingerprints (avoids stale geometry reuse). */
export function meshTransferContentKey(mesh: MeshTransfer, fallback = 0): string {
	const p = mesh.position;
	if (p.length === 0) return `empty-${fallback}`;
	const mid = ((p.length / 6) | 0) * 3;
	return `${p.length}-${p[0]}-${p[mid] ?? 0}-${p[p.length - 1] ?? 0}-${mesh.faceGroups.length}`;
}

export function isRenderableMeshTransfer(mesh: MeshTransfer): boolean {
	if (mesh.position.length === 0) return false;
	if (mesh.position.length % 3 !== 0) return false;
	if (mesh.normal.length !== mesh.position.length) return false;
	if (mesh.edges.length % 3 !== 0) return false;
	for (const value of mesh.position) {
		if (!Number.isFinite(value)) return false;
	}
	for (const value of mesh.normal) {
		if (!Number.isFinite(value)) return false;
	}
	for (const value of mesh.edges) {
		if (!Number.isFinite(value)) return false;
	}
	const vertexCount = mesh.position.length / 3;
	for (const value of mesh.index) {
		if (!Number.isFinite(value) || value < 0 || value >= vertexCount) return false;
	}
	return true;
}

/** @emoji 🎞️ Tessellates every model solid through `SpatialKernel.tessellate` (worker-backed). */
export function useDocumentMeshes(
	kernel: SpatialKernel | null,
	model: Model,
	tolerance: number,
): readonly { readonly solid: SolidRef; readonly mesh: MeshTransfer }[] {
	const [meshes, setMeshes] = useState<readonly { readonly solid: SolidRef; readonly mesh: MeshTransfer }[]>([]);
	const revision = model.revision;
	useEffect(() => {
		if (!kernel) {
			setMeshes([]);
			return;
		}
		const solids = listModelSolidRefs(model);
		if (solids.length === 0) {
			setMeshes([]);
			return;
		}
		let cancelled = false;
		void (async () => {
			const rows = await Promise.all(
				solids.map(async (solid) => {
					try {
						const mesh = await kernel.tessellate(solid, tolerance, model);
						return isRenderableMeshTransfer(mesh) ? { solid, mesh } : null;
					} catch {
						return null;
					}
				}),
			);
			if (!cancelled) setMeshes(rows.filter((row): row is { readonly solid: SolidRef; readonly mesh: MeshTransfer } => row !== null));
		})();
		return () => {
			cancelled = true;
		};
	}, [kernel, revision, tolerance]);
	return meshes;
}

/** @emoji 📐 Axis-aligned bounds of all mesh positions (for camera auto-fit). */
export function boundsFromMeshTransfers(meshes: readonly MeshTransfer[]): { readonly center: Vec3; readonly radius: number } | null {
	if (meshes.length === 0) return null;
	let minX = Infinity;
	let minY = Infinity;
	let minZ = Infinity;
	let maxX = -Infinity;
	let maxY = -Infinity;
	let maxZ = -Infinity;
	let hasFinitePoint = false;
	for (const mesh of meshes) {
		const pos = mesh.position;
		for (let i = 0; i < pos.length; i += 3) {
			const x = pos[i]!;
			const y = pos[i + 1]!;
			const z = pos[i + 2]!;
			if (!Number.isFinite(x) || !Number.isFinite(y) || !Number.isFinite(z)) continue;
			hasFinitePoint = true;
			if (x < minX) minX = x;
			if (y < minY) minY = y;
			if (z < minZ) minZ = z;
			if (x > maxX) maxX = x;
			if (y > maxY) maxY = y;
			if (z > maxZ) maxZ = z;
		}
	}
	if (!hasFinitePoint) return null;
	const cx = (minX + maxX) / 2;
	const cy = (minY + maxY) / 2;
	const cz = (minZ + maxZ) / 2;
	const dx = maxX - minX;
	const dy = maxY - minY;
	const dz = maxZ - minZ;
	const radius = Math.sqrt(dx * dx + dy * dy + dz * dz) / 2;
	return { center: [cx, cy, cz], radius: Math.max(radius, 0.5) };
}

/** @emoji 📐 Axis-aligned bounds of geometry vertex positions (factory / REPL geometry auto-fit). */
export function boundsFromSpatialPickGeometry(
	geometry: SpatialPickGeometry | null | undefined,
): { readonly center: Vec3; readonly radius: number } | null {
	if (!geometry) return null;
	const buckets = geometryBuckets(geometry);
	const verts = geometryRecords(buckets.vertices);
	if (!verts.length) return null;
	let minX = Infinity;
	let minY = Infinity;
	let minZ = Infinity;
	let maxX = -Infinity;
	let maxY = -Infinity;
	let maxZ = -Infinity;
	for (const vertex of verts) {
		const [x, y, z] = vertex.position;
		if (x < minX) minX = x;
		if (y < minY) minY = y;
		if (z < minZ) minZ = z;
		if (x > maxX) maxX = x;
		if (y > maxY) maxY = y;
		if (z > maxZ) maxZ = z;
	}
	const cx = (minX + maxX) / 2;
	const cy = (minY + maxY) / 2;
	const cz = (minZ + maxZ) / 2;
	const dx = maxX - minX;
	const dy = maxY - minY;
	const dz = maxZ - minZ;
	const radius = Math.sqrt(dx * dx + dy * dy + dz * dz) / 2;
	return { center: [cx, cy, cz], radius: Math.max(radius, 0.5) };
}

function mergeSpatialSceneBounds(
	a: { readonly center: Vec3; readonly radius: number } | null,
	b: { readonly center: Vec3; readonly radius: number } | null,
): { readonly center: Vec3; readonly radius: number } | null {
	if (!a) return b;
	if (!b) return a;
	const min: Vec3 = [
		Math.min(a.center[0] - a.radius, b.center[0] - b.radius),
		Math.min(a.center[1] - a.radius, b.center[1] - b.radius),
		Math.min(a.center[2] - a.radius, b.center[2] - b.radius),
	];
	const max: Vec3 = [
		Math.max(a.center[0] + a.radius, b.center[0] + b.radius),
		Math.max(a.center[1] + a.radius, b.center[1] + b.radius),
		Math.max(a.center[2] + a.radius, b.center[2] + b.radius),
	];
	const center: Vec3 = [(min[0] + max[0]) / 2, (min[1] + max[1]) / 2, (min[2] + max[2]) / 2];
	const radius = Math.max(
		Math.sqrt((max[0] - min[0]) ** 2 + (max[1] - min[1]) ** 2 + (max[2] - min[2]) ** 2) / 2,
		0.5,
	);
	return { center, radius };
}
// #endregion 🎬WorkerClient

// #region 🪩ArchivedFootprints
/** @emoji 📦 Footprint of a finished axis-aligned box for persistent REPL overlays. */
export interface ArchivedBoxLayout {
	readonly cornerA: Vec3;
	readonly cornerB: Vec3;
	readonly height: number;
}

function isVec3Record(v: unknown): v is Vec3 {
	return Array.isArray(v) && v.length === 3 && v.every((x) => typeof x === "number");
}

/** @emoji 📦 Reads `origin`/`corner`/`height` from post-commit interaction context when present. */
export function tryArchivedBoxFromContext(ctx: Record<string, unknown>): ArchivedBoxLayout | null {
	const o = ctx.origin;
	const c = ctx.corner;
	const h = ctx.height;
	if (!isVec3Record(o) || !isVec3Record(c)) return null;
	const hz = typeof h === "number" && Number.isFinite(h) && h > 0 ? h : null;
	if (hz === null) return null;
	return { cornerA: o, cornerB: c, height: hz };
}

function mergeDisplayWithArchivedBoxes(base: DisplayModel, archived: readonly ArchivedBoxLayout[]): DisplayModel {
	if (archived.length === 0) return base;
	const extra: DisplayItem[] = archived.map((b, i) => ({
		kind: "box-preview",
		id: `archived-box-${i}`,
		role: "archived",
		params: { cornerA: b.cornerA, cornerB: b.cornerB, height: b.height },
	}));
	return { ...base, items: [...extra, ...base.items] };
}

function archivedBoxesFromHistory(history: DocumentHistory): readonly ArchivedBoxLayout[] {
	return history
		.entries()
		.map((mod) => (mod.result.archiveContext ? tryArchivedBoxFromContext(mod.result.archiveContext) : null))
		.filter((box): box is ArchivedBoxLayout => box !== null);
}

function replBaseDisplayForHistory(snapshot: InteractionSnapshot): DisplayModel {
	if (snapshot.state !== "committed") return snapshot.display;
	return { ...snapshot.display, items: snapshot.display.items.filter((item) => item.role !== "preview") };
}
// #endregion 🪩ArchivedFootprints

// #region 📐Layout
/** @emoji 📐 Center and axis-aligned scale for a unit `BoxGeometry` from two XY footprint corners and height. */
export function computeBoxPreviewLayout(
	cornerA: Vec3,
	cornerB: Vec3,
	height: number,
	preview: SpatialPreviewKernel = scenePreview(),
): { readonly position: Vec3; readonly scale: Vec3 } {
	return preview.computeBoxPreviewLayout(cornerA, cornerB, height);
}

/** @emoji 🟦 Center and radius for the live sphere preview while the radius point is moving. */
export function computeSpherePreviewLayout(center: Vec3 | null, cursor: Vec3 | null): { readonly position: Vec3; readonly radius: number } | null {
	if (!center || !cursor) return null;
	const radius = Math.hypot(cursor[0] - center[0], cursor[1] - center[1], cursor[2] - center[2]);
	return radius > 1e-9 ? { position: center, radius } : null;
}

function readVec3(v: unknown): Vec3 | null {
	if (Array.isArray(v) && v.length === 3 && v.every((x) => typeof x === "number")) return v as unknown as Vec3;
	return null;
}

function readNumber(v: unknown): number | null {
	return typeof v === "number" && Number.isFinite(v) ? v : null;
}

function readVec3Array(v: unknown): readonly Vec3[] {
	if (!Array.isArray(v)) return [];
	return v.filter(isVec3Record) as readonly Vec3[];
}

/** @emoji 📦 Axis-aligned bounds for geometry highlight wireframes. */
export function bboxFromPoints(
	points: readonly Vec3[],
	preview: SpatialPreviewKernel = scenePreview(),
): { readonly min: Vec3; readonly max: Vec3 } | null {
	return preview.aabbFromPoints(points);
}

/** @emoji 📦 Twelve edges of an axis-aligned box for preview line rendering. */
export function bboxWireSegments(min: Vec3, max: Vec3): readonly (readonly [Vec3, Vec3])[] {
	const [x0, y0, z0] = min;
	const [x1, y1, z1] = max;
	const c: readonly Vec3[] = [
		[x0, y0, z0],
		[x1, y0, z0],
		[x1, y1, z0],
		[x0, y1, z0],
		[x0, y0, z1],
		[x1, y0, z1],
		[x1, y1, z1],
		[x0, y1, z1],
	];
	const idx: readonly (readonly [number, number])[] = [
		[0, 1],
		[1, 2],
		[2, 3],
		[3, 0],
		[4, 5],
		[5, 6],
		[6, 7],
		[7, 4],
		[0, 4],
		[1, 5],
		[2, 6],
		[3, 7],
	];
	return idx.map(([a, b]) => [c[a]!, c[b]!] as const);
}

function parseDisplaySelectionTargets(v: unknown): readonly { readonly kind: ModelEntityKind; readonly id: string }[] {
	if (!Array.isArray(v)) return [];
	const out: { kind: ModelEntityKind; id: string }[] = [];
	for (const raw of v) {
		if (!raw || typeof raw !== "object") continue;
		const o = raw as Record<string, unknown>;
		const kind = o.kind;
		const id = o.id;
		if (typeof kind === "string" && typeof id === "string") out.push({ kind: kind as ModelEntityKind, id });
	}
	return out;
}

/** @emoji 🖼️ Maps declarative `previewKind` + params to a point transform for geometry wireframes. */
export function transformPointsForPreviewKind(
	previewKind: string,
	params: Record<string, unknown>,
	preview: SpatialPreviewKernel = scenePreview(),
): (point: Vec3) => Vec3 {
	return preview.transformPointsForPreviewKind(previewKind, params);
}

/** @emoji 🖼️ Active geometry point transform from move/copy/mirror/rotate/scale preview display items. */
export function geometryPreviewTransformFromDisplay(model: DisplayModel): ((point: Vec3) => Vec3) | null {
	for (const item of model.items) {
		if (item.kind !== "preview" || !item.params) continue;
		const previewKind = typeof item.params.previewKind === "string" ? item.params.previewKind : "";
		if (!previewKindUsesGeometryWireframe(previewKind)) continue;
		if (
			previewKind === "move-preview" ||
			previewKind === "copy-preview" ||
			previewKind === "mirror-preview" ||
			previewKind === "rotate-preview" ||
			previewKind === "scale-preview" ||
			previewKind === "scale1d-preview"
		) {
			return transformPointsForPreviewKind(previewKind, item.params);
		}
	}
	return null;
}

function previewKindUsesGeometryWireframe(previewKind: string): boolean {
	return (
		previewKind === "selected-objects" ||
		previewKind === "move-preview" ||
		previewKind === "copy-preview" ||
		previewKind === "mirror-preview" ||
		previewKind === "rotate-preview" ||
		previewKind === "scale-preview" ||
		previewKind === "scale1d-preview" ||
		previewKind.endsWith("-selection") ||
		previewKind.startsWith("boolean-") ||
		previewKind === "highlight-curves" ||
		previewKind === "cutters" ||
		previewKind === "split-objects" ||
		previewKind === "trim-preview" ||
		previewKind === "extrusion" ||
		previewKind === "network-curves"
	);
}

const raycastNone: THREE.Object3D["raycast"] = () => undefined;
// #endregion 📐Layout

// #region 🧲GeometryTargets
export type SpatialPickKind = "pointer.down" | "pointer.move";

/** @emoji 🎯 Primitive and object pick kinds for renderer feedback (maps to kernel geometry via {@link SpatialPickTarget.geometryKind}). */
export type SpatialGeometryPickTargetKind = "object" | "face" | "edge" | "vertex";

export type SpatialPickTargetKind = SpatialGeometryPickTargetKind;

export const SPATIAL_PICK_TARGET_KINDS: readonly SpatialPickTargetKind[] = [
	"object",
	"face",
	"edge",
	"vertex",
];

const GEOMETRY_KIND_TO_OBJECT_PICK: Partial<Record<ModelEntityKind, SpatialGeometryPickTargetKind>> = {
	vertex: "vertex",
	edge: "edge",
	wire: "edge",
	face: "face",
	solid: "object",
	anchor: "vertex",
};

function spatialPickKindsForSelectionAccept(accept: readonly ModelEntityKind[]): ReadonlySet<SpatialPickTargetKind> | null {
	if (!accept.length) return null;
	const out = new Set<SpatialPickTargetKind>();
	for (const kind of accept) {
		if (
			kind === "object" ||
			kind === "face" ||
			kind === "edge" ||
			kind === "vertex"
		) {
			out.add(kind);
			continue;
		}
		const mapped = GEOMETRY_KIND_TO_OBJECT_PICK[kind];
		if (mapped) out.add(mapped);
		if (kind === "object") out.add("object");
	}
	return out;
}

function kernelGeometryKindForObjectPick(
	kind: SpatialGeometryPickTargetKind,
	geometryKind?: ModelEntityKind,
): ModelEntityKind {
	if (geometryKind) return geometryKind;
	if (kind === "vertex") return "vertex";
	if (kind === "edge") return "edge";
	if (kind === "face") return "face";
	return "solid";
}

/** @emoji 👁️ Per-kind on/off map for visibility filters or selection/hover gates (`false` disables). */
export type SpatialPickKindToggles = Partial<Record<SpatialPickTargetKind, boolean>>;

export interface SpatialPickTarget {
	readonly kind: SpatialPickTargetKind;
	readonly id: string;
	readonly point: Vec3;
	readonly points?: readonly Vec3[];
	/** @emoji 🧭 Kernel-private geometry entity kind for primitive picks (e.g. `wire` vs `edge`). */
	readonly geometryKind?: ModelEntityKind;
}

export interface SpatialSelectionRequest {
	readonly targets: readonly SpatialPickTarget[];
	readonly point: Vec3;
	readonly client: { readonly x: number; readonly y: number };
	readonly modifiers: InteractionEvent["modifiers"];
}

export type SpatialSelectionMethod = "rectangle" | "lasso";
type SpatialSelectionCoverage = "partial" | "full";
type SpatialSelectionMode = "default" | "additive" | "subtractive" | "invertive";

export interface SpatialDragSelectionState {
	readonly method: SpatialSelectionMethod;
	readonly coverage: SpatialSelectionCoverage;
	readonly startClient: { readonly x: number; readonly y: number };
	readonly currentClient: { readonly x: number; readonly y: number };
	readonly path: readonly { readonly x: number; readonly y: number }[];
	readonly modifiers: InteractionEvent["modifiers"];
}

export type SpatialPickGeometry = Model | ModelJson;

export function spatialPickTargetKey(target: SpatialPickTarget): string {
	return `${target.kind}:${target.id}`;
}

function spatialSelectionTargetKey(target: SelectionTarget): string {
	return `${target.kind}:${target.id}`;
}

/** @emoji 👁️ Default all geometry pick kinds enabled (visibility + selection). */
export function defaultSpatialPickKindToggles(): Record<SpatialPickTargetKind, boolean> {
	return Object.fromEntries(SPATIAL_PICK_TARGET_KINDS.map((kind) => [kind, true])) as Record<SpatialPickTargetKind, boolean>;
}

/** @emoji 👁️ Filters pick targets by visibility (show/hide highlights); does not affect ray pick or selection. */
export function filterSpatialPickTargetsForVisibility(
	targets: readonly SpatialPickTarget[],
	filterKindToggles: SpatialPickKindToggles = {},
): SpatialPickTarget[] {
	return targets.filter((target) => filterKindToggles[target.kind] !== false);
}

/** @emoji 👁️ Effective pick kinds must be both visible and enabled for selection/hover. */
export function intersectSpatialPickKindToggles(
	visibleKindToggles: SpatialPickKindToggles = {},
	selectionKindToggles: SpatialPickKindToggles = {},
): SpatialPickKindToggles {
	const merged: SpatialPickKindToggles = {};
	for (const kind of SPATIAL_PICK_TARGET_KINDS) {
		if (visibleKindToggles[kind] === false || selectionKindToggles[kind] === false) merged[kind] = false;
	}
	return merged;
}

/** @emoji 👁️ Maps active model-definition entity kinds to renderer pick-kind toggles. */
export function modelDefinitionPickTargetKinds(modelDefinitionId: string | null): readonly SpatialPickTargetKind[] {
	const entityKinds = modelDefinitionSelectionEntityKinds(modelDefinitionId ?? GEOMETRY_MODEL_DEFINITION_ID);
	const out = new Set<SpatialPickTargetKind>();
	for (const kind of entityKinds) {
		if (kind === "vertex" || kind === "anchor") out.add("vertex");
		else if (kind === "edge" || kind === "wire") out.add("edge");
		else if (kind === "face" || kind === "shell") out.add("face");
		else if (kind === "solid" || kind === "geometry" || kind === "object") out.add("object");
	}
	if (out.size > 0) return [...out];
	return isGeometryModelDefinition(modelDefinitionId) ? SPATIAL_PICK_TARGET_KINDS : ["object"];
}

/** @emoji 👁️ Default visibility/selection toggles for kinds allowed by the active model definition. */
export function defaultSpatialPickKindTogglesForModelDefinition(modelDefinitionId: string | null): Record<SpatialPickTargetKind, boolean> {
	const allowed = new Set(modelDefinitionPickTargetKinds(modelDefinitionId));
	return Object.fromEntries(SPATIAL_PICK_TARGET_KINDS.map((kind) => [kind, allowed.has(kind)])) as Record<SpatialPickTargetKind, boolean>;
}

/** @emoji 👁️ Resolves which scene layers stay visible for geometry edit vs typology object picking. */
export function resolveSpatialSceneVisibility(
	activeModelDefinitionId: string | null,
	filterKindToggles: SpatialPickKindToggles = {},
): {
	readonly showFactoryWireframe: boolean;
	readonly showCommittedFaces: boolean;
	readonly showCommittedEdges: boolean;
} {
	const visible = (kind: SpatialPickTargetKind) => filterKindToggles[kind] !== false;
	const mdId = activeModelDefinitionId ?? GEOMETRY_MODEL_DEFINITION_ID;
	if (isGeometryModelDefinition(mdId) || modelDefinitionUsesGeometryPicking(mdId)) {
		return {
			showFactoryWireframe: visible("edge"),
			showCommittedFaces: visible("face") || visible("object"),
			showCommittedEdges: visible("edge"),
		};
	}
	return {
		showFactoryWireframe: false,
		showCommittedFaces: false,
		showCommittedEdges: false,
	};
}

function spatialPickKindsForActiveView(activeModelDefinitionId: string | null): ReadonlySet<SpatialPickTargetKind> {
	return new Set(modelDefinitionPickTargetKinds(activeModelDefinitionId));
}

/** @emoji 👁️ Keeps pick targets allowed by the active model definition (topology + typology objects). */
export function filterSpatialPickTargetsForActiveView(
	targets: readonly SpatialPickTarget[],
	activeModelDefinitionId: string | null,
): SpatialPickTarget[] {
	const mdId = activeModelDefinitionId ?? GEOMETRY_MODEL_DEFINITION_ID;
	const allowedPickKinds = spatialPickKindsForActiveView(mdId);
	const entityKinds = new Set(modelDefinitionSelectionEntityKinds(mdId));
	return targets.filter((target) => {
		if (!allowedPickKinds.has(target.kind)) return false;
		if (target.kind === "object" && !target.geometryKind) return entityKinds.has("object");
		const geometryKind = target.geometryKind ?? kernelGeometryKindForObjectPick(target.kind, undefined);
		return entityKinds.has(geometryKind);
	});
}

function recordsById<T extends { id: string }>(xs: readonly T[]): Record<string, T> {
	const o: Record<string, T> = {};
	for (const x of xs) o[x.id] = x;
	return o;
}

function asRecordBucket<T extends { id: string }>(x: readonly T[] | Record<string, T> | undefined): Record<string, T> {
	if (!x) return {};
	return Array.isArray(x) ? recordsById(x) : (x as Record<string, T>);
}

/** @emoji 🧲 Normalizes `ModelJson` array buckets to the record shape used by interaction math. */
function geometryBuckets(g: SpatialPickGeometry): {
	readonly anchors: Record<string, AnchorRecord>;
	readonly vertices: Record<string, VertexRecord>;
	readonly edges: Record<string, EdgeRecord>;
	readonly wires: Record<string, WireRecord>;
	readonly faces: Record<string, FaceRecord>;
	readonly shells: Record<string, ShellRecord>;
	readonly solids: Record<string, SolidRecord>;
} {
	if (g instanceof Model) {
		return {
			anchors: g.anchors,
			vertices: g.vertices,
			edges: g.edges,
			wires: g.wires,
			faces: g.faces,
			shells: g.shells,
			solids: g.solids,
		};
	}
	return {
		anchors: asRecordBucket((g as ModelJson & { readonly anchors?: readonly AnchorRecord[] }).anchors),
		vertices: asRecordBucket(g.vertices),
		edges: asRecordBucket(g.edges),
		wires: asRecordBucket(g.wires),
		faces: asRecordBucket(g.faces),
		shells: asRecordBucket(g.shells),
		solids: asRecordBucket(g.solids),
	};
}

function geometryRecords<T>(records: Record<string, T> | undefined): readonly T[] {
	return records ? Object.values(records) : [];
}

function geometryPointCentroid(points: readonly Vec3[]): Vec3 | null {
	if (points.length === 0) return null;
	const sum = points.reduce(
		(acc, p) => [acc[0] + p[0], acc[1] + p[1], acc[2] + p[2]] as unknown as Vec3,
		[0, 0, 0] as unknown as Vec3,
	);
	return [sum[0] / points.length, sum[1] / points.length, sum[2] / points.length] as unknown as Vec3;
}

function geometryEdgePoints(vertices: Record<string, VertexRecord>, edge: EdgeRecord): readonly Vec3[] {
	return scenePreview().edgeSamplePoints(vertices, edge, 32);
}

function geometryFacePoints(
	vertices: Record<string, VertexRecord>,
	edges: Record<string, EdgeRecord>,
	wires: Record<string, WireRecord>,
	face: FaceRecord,
): readonly Vec3[] {
	const ids = face.wireIds.flatMap((wireId) => wires[wireId]?.edgeIds ?? []);
	const points = ids.flatMap((id) => {
		const edge = edges[id];
		return edge ? geometryEdgePoints(vertices, edge) : [];
	});
	const unique = new Map(points.map((p) => [p.join(","), p]));
	return [...unique.values()];
}

function uniqueGeometryPoints(points: readonly Vec3[]): readonly Vec3[] {
	return [...new Map(points.map((p) => [p.join(","), p])).values()];
}

function geometryWirePoints(vertices: Record<string, VertexRecord>, edges: Record<string, EdgeRecord>, wire: WireRecord): readonly Vec3[] {
	return uniqueGeometryPoints(wire.edgeIds.flatMap((id) => (edges[id] ? geometryEdgePoints(vertices, edges[id]!) : [])));
}

function geometryShellPoints(
	vertices: Record<string, VertexRecord>,
	edges: Record<string, EdgeRecord>,
	wires: Record<string, WireRecord>,
	faces: Record<string, FaceRecord>,
	shell: ShellRecord,
): readonly Vec3[] {
	return uniqueGeometryPoints(
		shell.faceIds.flatMap((id) => (faces[id] ? geometryFacePoints(vertices, edges, wires, faces[id]!) : [])),
	);
}

function geometrySolidPoints(
	vertices: Record<string, VertexRecord>,
	edges: Record<string, EdgeRecord>,
	wires: Record<string, WireRecord>,
	faces: Record<string, FaceRecord>,
	shells: Record<string, ShellRecord>,
	solid: SolidRecord,
): readonly Vec3[] {
	return uniqueGeometryPoints(
		solid.shellIds.flatMap((id) => (shells[id] ? geometryShellPoints(vertices, edges, wires, faces, shells[id]!) : [])),
	);
}

function geometryAllVertexPoints(vertices: Record<string, VertexRecord>): readonly Vec3[] {
	return geometryRecords(vertices).map((vertex) => vertex.position);
}

function geometryEntityPoints(
	buckets: ReturnType<typeof geometryBuckets>,
	kind: ModelEntityKind,
	id: string,
): readonly Vec3[] {
	if (kind === "anchor") {
		const anchor = buckets.anchors[id];
		return anchor ? [anchor.position] : [];
	}
	if (kind === "vertex") return buckets.vertices[id]?.position ? [buckets.vertices[id]!.position] : [];
	if (kind === "edge" && buckets.edges[id]) return geometryEdgePoints(buckets.vertices, buckets.edges[id]!);
	if (kind === "wire" && buckets.wires[id]) return geometryWirePoints(buckets.vertices, buckets.edges, buckets.wires[id]!);
	if (kind === "face" && buckets.faces[id]) return geometryFacePoints(buckets.vertices, buckets.edges, buckets.wires, buckets.faces[id]!);
	if (kind === "shell" && buckets.shells[id]) return geometryShellPoints(buckets.vertices, buckets.edges, buckets.wires, buckets.faces, buckets.shells[id]!);
	if (kind === "solid" && buckets.solids[id]) return geometrySolidPoints(buckets.vertices, buckets.edges, buckets.wires, buckets.faces, buckets.shells, buckets.solids[id]!);
	return [];
}

function geometryEntityPointsForPickTarget(
	buckets: ReturnType<typeof geometryBuckets>,
	target: SpatialPickTarget,
): readonly Vec3[] {
	const geometryKind = kernelGeometryKindForObjectPick(
		target.kind as SpatialGeometryPickTargetKind,
		target.geometryKind,
	);
	return geometryEntityPoints(buckets, geometryKind, target.id);
}

function geometryWireEdgeSegments(
	vertices: Record<string, VertexRecord>,
	edges: Record<string, EdgeRecord>,
	wire: WireRecord,
): readonly (readonly [Vec3, Vec3])[] {
	const out: (readonly [Vec3, Vec3])[] = [];
	for (const edgeId of wire.edgeIds) {
		const edge = edges[edgeId];
		if (!edge) continue;
		const pts = geometryEdgePoints(vertices, edge);
		if (pts.length >= 2) out.push([pts[0]!, pts[1]!]);
	}
	return out;
}

/** @emoji 📐 Geometry wire segments for previews (edges/wires/faces), bbox fallback for aggregates. */
export function geometryEntityWireSegments(
	buckets: ReturnType<typeof geometryBuckets>,
	kind: ModelEntityKind,
	id: string,
): readonly (readonly [Vec3, Vec3])[] {
	if (kind === "edge" && buckets.edges[id]) {
		return geometryWireEdgeSegments(buckets.vertices, buckets.edges, { id, edgeIds: [id] } as unknown as WireRecord);
	}
	if (kind === "wire" && buckets.wires[id]) return geometryWireEdgeSegments(buckets.vertices, buckets.edges, buckets.wires[id]!);
	if (kind === "face" && buckets.faces[id]) {
		const face = buckets.faces[id]!;
		return face.wireIds.flatMap((wireId) => {
			const wire = buckets.wires[wireId];
			return wire ? geometryWireEdgeSegments(buckets.vertices, buckets.edges, wire) : [];
		});
	}
	if (kind === "shell" && buckets.shells[id]) {
		return buckets.shells[id]!.faceIds.flatMap((faceId) => geometryEntityWireSegments(buckets, "face", faceId));
	}
	if (kind === "solid" && buckets.solids[id]) {
		return buckets.solids[id]!.shellIds.flatMap((shellId) => geometryEntityWireSegments(buckets, "shell", shellId));
	}
	const pts = geometryEntityPoints(buckets, kind, id);
	const bb = bboxFromPoints(pts);
	return bb ? bboxWireSegments(bb.min, bb.max) : [];
}

/** @emoji 📐 All B-rep edge segments for factory geometry wireframe display. */
export function collectGeometryEdgeSegments(
	buckets: ReturnType<typeof geometryBuckets>,
): readonly (readonly [Vec3, Vec3])[] {
	const out: (readonly [Vec3, Vec3])[] = [];
	for (const edge of geometryRecords(buckets.edges)) {
		const pts = geometryEdgePoints(buckets.vertices, edge);
		if (pts.length >= 2) out.push([pts[0]!, pts[pts.length - 1]!]);
	}
	return out;
}

function modelObjectPickPoints(model: Model, row: SpatialObjectRecord): readonly Vec3[] {
	const buckets = geometryBuckets(model);
	const cellRef = Object.values(row.primitives)[0];
	const cell = cellRef ? buckets.solids[cellRef] : undefined;
	if (!cell) return [];
	return geometrySolidPoints(buckets.vertices, buckets.edges, buckets.wires, buckets.faces, buckets.shells, cell);
}

function createModelObjectSpatialPickTargets(model: Model, modelDefinitionId: string): readonly SpatialPickTarget[] {
	const typologyIds = new Set(listTypologiesForModelDefinition(modelDefinitionId).map((row) => row.id));
	const targets: SpatialPickTarget[] = [];
	for (const row of Object.values(model.objects)) {
		if (!typologyIds.has(row.typology)) continue;
		const points = modelObjectPickPoints(model, row);
		const point = geometryPointCentroid(points);
		if (!point) continue;
		targets.push({
			kind: "object",
			id: String(row.id),
			point,
			points: points.length ? points : undefined,
		});
	}
	return targets;
}

function pickKindEnabledForSelectionAccept(accept: readonly ModelEntityKind[], pickKind: SpatialPickTargetKind): boolean {
	if (accept.length === 0) return true;
	const mapped = spatialPickKindsForSelectionAccept(accept);
	return mapped ? mapped.has(pickKind) : true;
}

function appendTopologySpatialPickTargets(
	targets: SpatialPickTarget[],
	buckets: ReturnType<typeof geometryBuckets>,
	entityKinds: ReadonlySet<ModelEntityKind>,
): void {
	if (entityKinds.has("anchor")) {
		for (const anchor of geometryRecords(buckets.anchors)) {
			targets.push({ kind: "vertex", geometryKind: "anchor", id: anchor.id, point: anchor.position });
		}
	}
	if (entityKinds.has("vertex")) {
		for (const vertex of geometryRecords(buckets.vertices)) {
			targets.push({ kind: "vertex", geometryKind: "vertex", id: vertex.id, point: vertex.position });
		}
	}
	if (entityKinds.has("edge")) {
		for (const edge of geometryRecords(buckets.edges)) {
			const points = geometryEdgePoints(buckets.vertices, edge);
			const point = geometryPointCentroid(points);
			if (point) targets.push({ kind: "edge", geometryKind: "edge", id: edge.id, point, points });
		}
	}
	if (entityKinds.has("wire")) {
		for (const wire of geometryRecords(buckets.wires)) {
			const points = geometryWirePoints(buckets.vertices, buckets.edges, wire);
			const point = geometryPointCentroid(points);
			if (point) targets.push({ kind: "edge", geometryKind: "wire", id: wire.id, point, points });
		}
	}
	if (entityKinds.has("face")) {
		for (const face of geometryRecords(buckets.faces)) {
			const points = geometryFacePoints(buckets.vertices, buckets.edges, buckets.wires, face);
			const point = geometryPointCentroid(points);
			if (point) targets.push({ kind: "face", geometryKind: "face", id: face.id, point, points });
		}
	}
	if (entityKinds.has("shell")) {
		for (const shell of geometryRecords(buckets.shells)) {
			const segments = geometryEntityWireSegments(buckets, "shell", shell.id);
			const points = segments.flatMap(([a, b]) => [a, b]);
			const point = geometryPointCentroid(points);
			if (point) targets.push({ kind: "face", geometryKind: "shell", id: shell.id, point, points: points.length ? points : undefined });
		}
	}
	if (entityKinds.has("solid")) {
		const all = geometryAllVertexPoints(buckets.vertices);
		const allCenter = geometryPointCentroid(all);
		for (const cell of geometryRecords(buckets.solids)) {
			const points = geometrySolidPoints(buckets.vertices, buckets.edges, buckets.wires, buckets.faces, buckets.shells, cell);
			const point = geometryPointCentroid(points) ?? allCenter;
			if (point) targets.push({ kind: "object", geometryKind: "solid", id: cell.id, point, points: points.length ? points : all });
		}
	}
}

/** @emoji 🧲 Builds renderer-side snap/select targets from factory geometry and typology object rows. */
export function createSpatialPickTargets(
	geometry: SpatialPickGeometry | null | undefined,
	activeModelDefinitionId?: string | null,
): readonly SpatialPickTarget[] {
	if (!geometry) return [];
	const buckets = geometryBuckets(geometry);
	const model = geometry instanceof Model ? geometry : parseModelJson(geometry as ModelJson);
	if (!model) return [];
	const mdId = activeModelDefinitionId ?? GEOMETRY_MODEL_DEFINITION_ID;
	const entityKinds = new Set(modelDefinitionSelectionEntityKinds(mdId));
	const targets: SpatialPickTarget[] = [];
	if (modelDefinitionUsesGeometryPicking(mdId)) appendTopologySpatialPickTargets(targets, buckets, entityKinds);
	if (entityKinds.has("object") && !isGeometryModelDefinition(mdId)) targets.push(...createModelObjectSpatialPickTargets(model, mdId));
	return targets;
}

export function filterSpatialPickTargets(
	targets: readonly SpatialPickTarget[],
	accept: readonly ModelEntityKind[] = [],
	toggles: SpatialPickKindToggles = {},
): SpatialPickTarget[] {
	const acceptKinds = spatialPickKindsForSelectionAccept(accept);
	return targets.filter(
		(target) => toggles[target.kind] !== false && (!acceptKinds || acceptKinds.has(target.kind)),
	);
}

/** @emoji 🧲 Creates a statechart event carrying snapped point plus selected geometry metadata. */
export function createSpatialPickEvent(
	kind: SpatialPickKind,
	point: Vec3,
	target: SpatialPickTarget | null,
	modifiers: InteractionEvent["modifiers"] = {},
): InteractionEvent {
	const geometryKind =
		target?.kind === "object" && !target.geometryKind
			? "object"
			: target
				? kernelGeometryKindForObjectPick(target.kind as SpatialGeometryPickTargetKind, target.geometryKind)
				: undefined;
	return target && geometryKind
		? {
				kind,
				point,
				modifiers,
				snap: { kind: geometryKind, id: target.id, point: target.point },
				selection: { kind: geometryKind, id: target.id },
			}
		: { kind, point, modifiers };
}
// #endregion 🧲GeometryTargets

// #region 🖼️DisplayPrimitives
function BoxPreviewItem({ item }: { readonly item: DisplayItem }): ReactNode {
	const p = item.params;
	const edgeGeo = useMemo(() => new THREE.EdgesGeometry(new THREE.BoxGeometry(1, 1, 1)), []);
	if (!p) return null;
	const a = readVec3(p.cornerA);
	const b = readVec3(p.cornerB);
	const hRaw = readNumber(p.height);
	if (!a || !b) return null;
	const h = hRaw === null || hRaw <= 0 ? 0.06 : hRaw;
	const { position, scale } = computeBoxPreviewLayout(a, b, h);
	const archived = item.role === "archived";
	return (
		<group position={position} scale={scale}>
			<mesh raycast={raycastNone}>
				<boxGeometry args={[1, 1, 1]} />
				<meshStandardMaterial
					color={archived ? "#5a8c6a" : "#7ab0ff"}
					emissive={archived ? "#0a2818" : "#102a66"}
					emissiveIntensity={archived ? 0.22 : 0.35}
					transparent
					opacity={archived ? 0.38 : 0.52}
					depthWrite={false}
				/>
			</mesh>
			<lineSegments raycast={raycastNone} geometry={edgeGeo}>
				<lineBasicMaterial color={archived ? "#a8d4b8" : "#ffffff"} transparent opacity={archived ? 0.55 : 0.85} />
			</lineSegments>
		</group>
	);
}

function PointItem({ item }: { readonly item: DisplayItem }): ReactNode {
	const pos = readVec3(item.params?.position);
	if (!pos) return null;
	const cursor = item.role === "cursor";
	const r = cursor ? 0.045 : 0.06;
	return (
		<mesh position={pos} raycast={raycastNone}>
			<sphereGeometry args={[r, 16, 16]} />
			<meshStandardMaterial
				color={cursor ? "#66e8ff" : "#ffcc66"}
				emissive={cursor ? "#003844" : "#553300"}
				emissiveIntensity={cursor ? 0.45 : 0.35}
			/>
		</mesh>
	);
}

function LinearHandleItem({ item }: { readonly item: DisplayItem }): ReactNode {
	const p = item.params;
	if (!p) return null;
	const origin = readVec3(p.origin);
	const axis = readVec3(p.axis);
	if (!origin || !axis) return null;
	const ax = axis[0];
	const ay = axis[1];
	const az = axis[2];
	const len = Math.hypot(ax, ay, az) || 1;
	const ux = ax / len;
	const uy = ay / len;
	const uz = az / len;
	const span = 5;
	const x1 = origin[0] + ux * span;
	const y1 = origin[1] + uy * span;
	const z1 = origin[2] + uz * span;
	return (
		<Line
			raycast={raycastNone}
			points={[
				[origin[0], origin[1], origin[2]],
				[x1, y1, z1],
			]}
			color="#ffff88"
			lineWidth={2}
			dashed={false}
		/>
	);
}

function SegmentItem({ item }: { readonly item: DisplayItem }): ReactNode {
	const p = item.params;
	if (!p) return null;
	const a = readVec3(p.from);
	const b = readVec3(p.to);
	if (!a || !b) return null;
	return (
		<Line
			raycast={raycastNone}
			points={[
				[a[0], a[1], a[2]],
				[b[0], b[1], b[2]],
			]}
			color="#88eeff"
			lineWidth={2}
			dashed={false}
		/>
	);
}

function LabelItem({ item }: { readonly item: DisplayItem }): ReactNode {
	const p = item.params;
	if (!p) return null;
	const pos = readVec3(p.position);
	const text = p.text;
	if (!pos || typeof text !== "string") return null;
	return (
		<Suspense fallback={null}>
			<Text position={pos} fontSize={0.22} color="#f4f4ff" anchorX="left" anchorY="bottom" raycast={raycastNone}>
				{text}
			</Text>
		</Suspense>
	);
}

function GeometryTargetWireframes({
	geometry,
	targets,
	transform,
	color,
	opacity,
}: {
	readonly geometry: SpatialPickGeometry;
	readonly targets: readonly { readonly kind: ModelEntityKind; readonly id: string }[];
	readonly transform: (point: Vec3) => Vec3;
	readonly color: string;
	readonly opacity: number;
}): ReactNode {
	const buckets = useMemo(() => geometryBuckets(geometry), [geometry]);
	const segments = useMemo(() => {
		const out: (readonly [Vec3, Vec3])[] = [];
		for (const target of targets) {
			for (const [a, b] of geometryEntityWireSegments(buckets, target.kind, target.id)) {
				out.push([transform(a), transform(b)]);
			}
		}
		return out;
	}, [buckets, targets, transform]);
	if (!segments.length) return null;
	return (
		<group>
			{segments.map(([a, b], i) => (
				<Line
					key={`${a[0]}-${a[1]}-${a[2]}-${b[0]}-${b[1]}-${b[2]}-${i}`}
					raycast={raycastNone}
					points={[
						[a[0], a[1], a[2]],
						[b[0], b[1], b[2]],
					]}
					color={color}
					lineWidth={2}
					transparent
					opacity={opacity}
				/>
			))}
		</group>
	);
}

function GeometryTargetPreviewMeshes({
	geometry,
	targets,
	transform,
	color,
	opacity,
}: {
	readonly geometry: SpatialPickGeometry;
	readonly targets: readonly { readonly kind: ModelEntityKind; readonly id: string }[];
	readonly transform: (point: Vec3) => Vec3;
	readonly color: string;
	readonly opacity: number;
}): ReactNode {
	const buckets = useMemo(() => geometryBuckets(geometry), [geometry]);
	const solids = useMemo(() => {
		const out: { readonly key: string; readonly center: Vec3; readonly size: Vec3 }[] = [];
		for (const target of targets) {
			const pts = geometryEntityPointsForPickTarget(buckets, target).map(transform);
			if (target.kind === "vertex" && pts[0]) {
				out.push({ key: `${target.kind}:${target.id}:v`, center: pts[0], size: [0.1, 0.1, 0.1] });
				continue;
			}
			const bounds = targetBounds(pts);
			if (!bounds) continue;
			out.push({ key: `${target.kind}:${target.id}`, center: bounds.center, size: bounds.size });
		}
		return out;
	}, [buckets, targets, transform]);
	if (!solids.length) return null;
	return (
		<group>
			{solids.map((solid) => (
				<mesh key={solid.key} position={solid.center} scale={solid.size} raycast={raycastNone}>
					<boxGeometry args={[1, 1, 1]} />
					<meshStandardMaterial
						color={color}
						emissive={color}
						emissiveIntensity={0.12}
						transparent
						opacity={opacity}
						depthWrite={false}
						side={THREE.DoubleSide}
					/>
				</mesh>
			))}
		</group>
	);
}

function PreviewItem({
	item,
	geometry,
}: {
	readonly item: DisplayItem;
	readonly geometry?: SpatialPickGeometry | null;
}): ReactNode {
	const p = item.params;
	if (!p) return null;
	const previewKind = typeof p.previewKind === "string" ? p.previewKind : "preview";
	const targets = parseDisplaySelectionTargets(p.targets);
	const transform = useMemo(() => transformPointsForPreviewKind(previewKind, p), [previewKind, p]);
	const points = readVec3Array(p.points);
	const cursor = readVec3(p.cursor);
	const prevPoint = readVec3(p.prevPoint);
	const from = readVec3(p.from) ?? prevPoint;
	const linePoints = points.length ? [...points, ...(cursor ? [cursor] : [])] : from && cursor ? [from, cursor] : [];
	const ghost =
		previewKind === "move-preview" || previewKind === "copy-preview" || previewKind === "mirror-preview";
	const wireColor =
		previewKind === "selected-objects" || previewKind.endsWith("-selection") ? "#ffcc66" : ghost ? "#7ab0ff" : "#88eeff";
	const wireOpacity = ghost ? 0.92 : 0.78;
	const meshColor = ghost ? "#4a6088" : wireColor;
	const meshOpacity = ghost ? 0.28 : 0.42;
	if (geometry && targets.length && previewKindUsesGeometryWireframe(previewKind)) {
		return (
			<group>
				{ghost ? (
					<GeometryTargetWireframes
						geometry={geometry}
						targets={targets}
						transform={(pt) => pt}
						color="#4a6088"
						opacity={0.35}
					/>
				) : null}
				<GeometryTargetPreviewMeshes
					geometry={geometry}
					targets={targets}
					transform={transform}
					color={meshColor}
					opacity={meshOpacity}
				/>
				<GeometryTargetWireframes
					geometry={geometry}
					targets={targets}
					transform={transform}
					color={wireColor}
					opacity={wireOpacity}
				/>
				{from ? (
					<mesh position={from} raycast={raycastNone}>
						<sphereGeometry args={[0.05, 12, 12]} />
						<meshStandardMaterial color="#ff9966" emissive="#442200" emissiveIntensity={0.4} />
					</mesh>
				) : null}
				{linePoints.length >= 2 ? (
					<Line raycast={raycastNone} points={linePoints.map((pt) => [pt[0], pt[1], pt[2]])} color="#ffff88" lineWidth={2} />
				) : null}
			</group>
		);
	}
	if (previewKind === "sphere" && points.length >= 1 && cursor) {
		const sphere = computeSpherePreviewLayout(points[0]!, cursor);
		if (sphere) {
			return (
				<group>
					<mesh position={sphere.position} raycast={raycastNone}>
						<sphereGeometry args={[sphere.radius, 32, 16]} />
						<meshStandardMaterial
							color="#7ab0ff"
							emissive="#102a66"
							emissiveIntensity={0.28}
							transparent
							opacity={0.34}
							depthWrite={false}
							side={THREE.DoubleSide}
						/>
					</mesh>
					<mesh position={sphere.position} raycast={raycastNone}>
						<sphereGeometry args={[sphere.radius, 32, 16]} />
						<meshBasicMaterial color="#d7ecff" wireframe transparent opacity={0.55} depthWrite={false} />
					</mesh>
					<Line
						raycast={raycastNone}
						points={[[sphere.position[0], sphere.position[1], sphere.position[2]], [cursor[0], cursor[1], cursor[2]]]}
						color="#ffff88"
						lineWidth={1.5}
						dashed
						dashSize={0.08}
						gapSize={0.06}
					/>
					<mesh position={sphere.position} raycast={raycastNone}>
						<sphereGeometry args={[0.04, 10, 10]} />
						<meshStandardMaterial color="#ffcc66" emissive="#553300" emissiveIntensity={0.35} />
					</mesh>
				</group>
			);
		}
	}
	// #region 🔵CircleArcPreview
	if ((previewKind === "circle-outline" || previewKind === "circle") && points.length >= 1 && cursor) {
		const center = points[0]!;
		const radius = Math.hypot(cursor[0] - center[0], cursor[1] - center[1], cursor[2] - center[2]);
		if (radius > 1e-9) {
			const segments = 64;
			const circlePts: [number, number, number][] = [];
			for (let i = 0; i <= segments; i++) {
				const a = (i / segments) * Math.PI * 2;
				circlePts.push([center[0] + Math.cos(a) * radius, center[1] + Math.sin(a) * radius, center[2]]);
			}
			return (
				<group>
					<Line raycast={raycastNone} points={circlePts} color="#88eeff" lineWidth={2} />
					<Line
						raycast={raycastNone}
						points={[[center[0], center[1], center[2]], [cursor[0], cursor[1], cursor[2]]]}
						color="#ffff88"
						lineWidth={1.5}
						dashed
						dashSize={0.08}
						gapSize={0.06}
					/>
					<mesh position={center} raycast={raycastNone}>
						<sphereGeometry args={[0.04, 10, 10]} />
						<meshStandardMaterial color="#ffcc66" emissive="#553300" emissiveIntensity={0.35} />
					</mesh>
				</group>
			);
		}
	}
	if (previewKind === "arc" && points.length >= 2 && cursor) {
		const center = points[0]!;
		const start = points[1]!;
		const arcEnd = scenePreview().arcEndOnCircle(center, start, cursor);
		const arcPts = scenePreview().arcSamplePoints(center, start, arcEnd, 64);
		if (arcPts.length >= 2) {
			return (
				<group>
					<Line raycast={raycastNone} points={arcPts.map((pt) => [pt[0], pt[1], pt[2]])} color="#88eeff" lineWidth={2} />
					<Line
						raycast={raycastNone}
						points={[[center[0], center[1], center[2]], [start[0], start[1], start[2]]]}
						color="#ffff88"
						lineWidth={1.5}
						dashed
						dashSize={0.08}
						gapSize={0.06}
					/>
					<Line
						raycast={raycastNone}
						points={[[center[0], center[1], center[2]], [arcEnd[0], arcEnd[1], arcEnd[2]]]}
						color="#ffff88"
						lineWidth={1.5}
						dashed
						dashSize={0.08}
						gapSize={0.06}
					/>
					<mesh position={center} raycast={raycastNone}>
						<sphereGeometry args={[0.04, 10, 10]} />
						<meshStandardMaterial color="#ffcc66" emissive="#553300" emissiveIntensity={0.35} />
					</mesh>
					<mesh position={start} raycast={raycastNone}>
						<sphereGeometry args={[0.04, 10, 10]} />
						<meshStandardMaterial color="#ffcc66" emissive="#553300" emissiveIntensity={0.35} />
					</mesh>
					<mesh position={arcEnd} raycast={raycastNone}>
						<sphereGeometry args={[0.04, 10, 10]} />
						<meshStandardMaterial color="#88eeff" emissive="#113344" emissiveIntensity={0.35} />
					</mesh>
				</group>
			);
		}
	}
	// #endregion 🔵CircleArcPreview
	if (previewKind === "interpolated-curve" && linePoints.length >= 2) {
		const splinePoints = linePoints.map((pt) => new THREE.Vector3(pt[0], pt[1], pt[2]));
		const curve = new THREE.CatmullRomCurve3(splinePoints);
		const segments = Math.max(64, splinePoints.length * 16);
		const sampled = curve.getPoints(segments).map((v): [number, number, number] => [v.x, v.y, v.z]);
		const placedCount = cursor ? splinePoints.length - 1 : splinePoints.length;
		return (
			<group>
				<Line raycast={raycastNone} points={sampled} color="#88eeff" lineWidth={2} />
				{splinePoints.slice(0, placedCount).map((v, i) => (
					<mesh key={i} position={[v.x, v.y, v.z]} raycast={raycastNone}>
						<sphereGeometry args={[0.04, 10, 10]} />
						<meshStandardMaterial color="#ffcc66" emissive="#553300" emissiveIntensity={0.35} />
					</mesh>
				))}
			</group>
		);
	}
	return (
		<group>
			{linePoints.length >= 2 ? (
				<Line raycast={raycastNone} points={linePoints.map((pt) => [pt[0], pt[1], pt[2]])} color="#88eeff" lineWidth={2} />
			) : null}
		</group>
	);
}

function EntityHighlightItem({
	item,
	geometry,
}: {
	readonly item: DisplayItem;
	readonly geometry?: SpatialPickGeometry | null;
}): ReactNode {
	const p = item.params;
	if (!p || !geometry) return null;
	const entity = p.entity;
	if (!entity || typeof entity !== "object") return null;
	const kind = (entity as { kind?: unknown }).kind;
	const id = (entity as { id?: unknown }).id;
	if (typeof kind !== "string" || typeof id !== "string") return null;
	return (
		<GeometryTargetWireframes
			geometry={geometry}
			targets={[{ kind: kind as ModelEntityKind, id }]}
			transform={(pt) => pt}
			color="#ffcc66"
			opacity={0.85}
		/>
	);
}

function CurveItem({ item }: { readonly item: DisplayItem }): ReactNode {
	const points = readVec3Array(item.params?.points);
	if (points.length < 2) return null;
	return (
		<Line
			raycast={raycastNone}
			points={points.map((pt) => [pt[0], pt[1], pt[2]])}
			color="#88eeff"
			lineWidth={2}
		/>
	);
}

function isMeshTransferLike(v: unknown): v is MeshTransfer {
	if (!v || typeof v !== "object") return false;
	const m = v as MeshTransfer;
	return m.position instanceof Float32Array && m.index instanceof Uint32Array && Array.isArray(m.faceGroups);
}

function MeshItem({ item }: { readonly item: DisplayItem }): ReactNode {
	const raw = item.params?.mesh ?? item.params?.transfer;
	if (!isMeshTransferLike(raw)) return null;
	return <TessellatedCommitMesh mesh={raw} />;
}

function defaultDisplayItemNode(item: DisplayItem, geometry?: SpatialPickGeometry | null): ReactNode {
	switch (item.kind) {
		case "box-preview":
			return <BoxPreviewItem item={item} />;
		case "point":
			return <PointItem item={item} />;
		case "linear-handle":
			return <LinearHandleItem item={item} />;
		case "segment":
			return <SegmentItem item={item} />;
		case "label":
			return <LabelItem item={item} />;
		case "preview":
			return <PreviewItem item={item} geometry={geometry} />;
		case "entity-highlight":
			return <EntityHighlightItem item={item} geometry={geometry} />;
		case "curve":
			return <CurveItem item={item} />;
		case "mesh":
			return <MeshItem item={item} />;
		default:
			return null;
	}
}

// #region 🎨HostCustomization
/** @emoji 🖼️ Host hook that renders one resolved `DisplayItem` inside `<InteractionDisplay>`. */
export type SpatialDisplayItemRenderer = (
	item: DisplayItem,
	geometry: SpatialPickGeometry | null | undefined,
	defaultRender: () => ReactNode,
) => ReactNode;

const spatialDisplayItemRenderers = new Map<string, SpatialDisplayItemRenderer>();

/** @emoji 🖼️ Registers a custom display kind; returns unregister. Libraries extend without forking the package. */
export function registerSpatialDisplayItemKind(kind: string, render: SpatialDisplayItemRenderer): () => void {
	spatialDisplayItemRenderers.set(kind, render);
	return () => spatialDisplayItemRenderers.delete(kind);
}

/** @emoji 🖼️ Looks up a host-registered display kind renderer. */
export function getSpatialDisplayItemKindRenderer(kind: string): SpatialDisplayItemRenderer | undefined {
	return spatialDisplayItemRenderers.get(kind);
}

function renderDisplayItem(
	item: DisplayItem,
	geometry: SpatialPickGeometry | null | undefined,
	renderItem?: SpatialDisplayItemRenderer,
): ReactNode {
	const fallback = () => defaultDisplayItemNode(item, geometry);
	const custom = renderItem ?? spatialDisplayItemRenderers.get(item.kind);
	return custom ? custom(item, geometry, fallback) : fallback();
}

/** @emoji 🪩 Optional scene slots for host overlays (gizmos, annotations, alternate lighting). */
export interface InteractionSpatialViewSlots {
	readonly beforeScene?: ReactNode;
	readonly afterDisplay?: ReactNode;
	readonly afterCommitted?: ReactNode;
	readonly lights?: ReactNode;
	readonly environment?: ReactNode;
}

/** @emoji 🎨 Theme tokens for default scene chrome (hosts override per product). */
export interface InteractionSpatialViewTheme {
	readonly background?: string;
	readonly ambientIntensity?: number;
	readonly directionalIntensity?: number;
	readonly directionalPosition?: Vec3;
	readonly gridDivisions?: number;
	readonly gridSize?: number;
	readonly groundPlaneColor?: string;
	readonly groundPlaneOpacity?: number;
}

export const defaultInteractionSpatialViewTheme: InteractionSpatialViewTheme = {
	background: "#080810",
	ambientIntensity: 0.45,
	directionalIntensity: 1.1,
	directionalPosition: [12, 18, 10],
	gridDivisions: 40,
	gridSize: 40,
	groundPlaneColor: "#7a9dff",
	groundPlaneOpacity: 0.18,
};
// #endregion 🎨HostCustomization

/** @emoji 🖼️ Maps `DisplayModel.items` to R3F nodes (must live under `<Canvas>`). */
export function InteractionDisplay({
	model,
	geometry,
	renderItem,
}: {
	readonly model: DisplayModel;
	readonly geometry?: SpatialPickGeometry | null;
	readonly renderItem?: SpatialDisplayItemRenderer;
}): ReactNode {
	return (
		<group>
			{model.items.map((item) => (
				<group key={item.id}>{renderDisplayItem(item, geometry, renderItem)}</group>
			))}
		</group>
	);
}
// #endregion 🖼️DisplayPrimitives

// #region 🖱️Interaction
function pointerModifiers(event: ThreeEvent<PointerEvent>) {
	return {
		alt: event.altKey,
		ctrl: event.ctrlKey,
		meta: event.metaKey,
		shift: event.shiftKey,
	};
}

/** @emoji 🖱️ Ground hit-test on the **XY** working plane at fixed world **Z** (= spatial footprint plane; factory height is world Z). */
export interface GroundPickPlaneProps {
	readonly planeZ?: number;
	readonly enabled?: boolean;
	readonly onPick?: (point: Vec3) => void;
	readonly onContextPick?: (point: Vec3) => void;
	readonly onPointerMove?: (point: Vec3) => void;
	readonly pointerMoveEnabled?: boolean;
	readonly planeColor?: string;
	readonly planeOpacity?: number;
}

export function GroundPickPlane({
	planeZ = 0,
	enabled = true,
	onPick,
	onContextPick,
	onPointerMove,
	pointerMoveEnabled,
	planeColor = "#7a9dff",
	planeOpacity = 0.18,
}: GroundPickPlaneProps): ReactNode {
	const moveOn = pointerMoveEnabled ?? Boolean(onPointerMove);
	const onPointerDown = (e: ThreeEvent<PointerEvent>) => {
		if (!enabled || !onPick) return;
		e.stopPropagation();
		const p = e.point;
		onPick([p.x, p.y, planeZ] as unknown as Vec3);
	};
	const onContextMenu = (e: ThreeEvent<MouseEvent>) => {
		if (!enabled || !onContextPick) return;
		e.stopPropagation();
		const p = e.point;
		onContextPick([p.x, p.y, planeZ] as unknown as Vec3);
	};
	const onPointerMoveH = (e: ThreeEvent<PointerEvent>) => {
		if (!moveOn || !onPointerMove) return;
		e.stopPropagation();
		const p = e.point;
		onPointerMove([p.x, p.y, planeZ] as unknown as Vec3);
	};
	return (
		<mesh position={[0, 0, planeZ]} onPointerDown={onPointerDown} onContextMenu={onContextMenu} onPointerMove={onPointerMoveH}>
			<planeGeometry args={[120, 120]} />
			<meshBasicMaterial transparent opacity={planeOpacity} color={planeColor} side={THREE.DoubleSide} />
		</mesh>
	);
}

function vec3FromSnapshotContext(ctx: Record<string, unknown>, key: string): Vec3 | null {
	return readVec3(ctx[key]);
}

/** @emoji 🖱️ YZ wall at the second corner so `pointer.move` changes world Z (factory height uses |Δz|). */
function HeightDragSurface({
	origin,
	corner,
	enabled,
	onPointerMove,
}: {
	readonly origin: Vec3;
	readonly corner: Vec3;
	readonly enabled: boolean;
	readonly onPointerMove?: (point: Vec3) => void;
}): ReactNode {
	const z0 = origin[2];
	const zSpan = 10;
	const zMid = z0 + zSpan / 2;
	const ySpan = 6;
	const onMove = (e: ThreeEvent<PointerEvent>) => {
		if (!enabled || !onPointerMove) return;
		e.stopPropagation();
		const p = e.point;
		onPointerMove([p.x, p.y, p.z] as unknown as Vec3);
	};
	const xPlane = corner[0] + 0.06;
	return (
		<mesh
			position={[xPlane, corner[1], zMid]}
			rotation={[0, Math.PI / 2, 0]}
			onPointerMove={onMove}
			renderOrder={2}
		>
			<planeGeometry args={[zSpan, ySpan]} />
			<meshStandardMaterial
				transparent
				opacity={0.38}
				color="#3ecf9f"
				emissive="#0a3020"
				emissiveIntensity={0.25}
				roughness={0.88}
				metalness={0.08}
				depthWrite={false}
				side={THREE.DoubleSide}
			/>
		</mesh>
	);
}

/** @emoji 🖱️ Z-aligned rod at `origin` so `pointer.move` drives peak height without XY drift. */
function VerticalZDragRod({
	origin,
	enabled,
	onPointerMove,
}: {
	readonly origin: Vec3;
	readonly enabled: boolean;
	readonly onPointerMove?: (point: Vec3) => void;
}): ReactNode {
	const h = 22;
	const onMove = (e: ThreeEvent<PointerEvent>) => {
		if (!enabled || !onPointerMove) return;
		e.stopPropagation();
		const p = e.point;
		onPointerMove([p.x, p.y, p.z] as unknown as Vec3);
	};
	return (
		<mesh
			position={[origin[0], origin[1], origin[2] + h / 2]}
			rotation={[Math.PI / 2, 0, 0]}
			onPointerMove={onMove}
			renderOrder={3}
		>
			<cylinderGeometry args={[0.14, 0.14, h, 10]} />
			<meshStandardMaterial
				transparent
				opacity={0.14}
				color="#55aaff"
				depthWrite={false}
				side={THREE.DoubleSide}
			/>
		</mesh>
	);
}

/** @emoji 🎮 Maps R3F pointer events to `InteractionEvent` envelopes (point + modifiers). */
export function createR3FInteractionAdapter() {
	const toPoint = (event: ThreeEvent<PointerEvent>): Vec3 => [event.point.x, event.point.y, event.point.z];
	return {
		pointerMove: (event: ThreeEvent<PointerEvent>): InteractionEvent => ({
			kind: "pointer.move",
			point: toPoint(event),
			modifiers: pointerModifiers(event),
		}),
		pointerDown: (event: ThreeEvent<PointerEvent>): InteractionEvent => ({
			kind: "pointer.down",
			point: toPoint(event),
			modifiers: pointerModifiers(event),
		}),
	};
}
// #endregion 🖱️Interaction

// #region 🧲GeometryInteraction
function targetBounds(points: readonly Vec3[]): { readonly center: Vec3; readonly size: Vec3 } | null {
	if (points.length === 0) return null;
	const min = points.reduce(
		(acc, p) => [Math.min(acc[0], p[0]), Math.min(acc[1], p[1]), Math.min(acc[2], p[2])] as unknown as Vec3,
		points[0]!,
	);
	const max = points.reduce(
		(acc, p) => [Math.max(acc[0], p[0]), Math.max(acc[1], p[1]), Math.max(acc[2], p[2])] as unknown as Vec3,
		points[0]!,
	);
	return {
		center: [(min[0] + max[0]) / 2, (min[1] + max[1]) / 2, (min[2] + max[2]) / 2] as unknown as Vec3,
		size: [
			Math.max(max[0] - min[0], 0.08),
			Math.max(max[1] - min[1], 0.08),
			Math.max(max[2] - min[2], 0.08),
		] as unknown as Vec3,
	};
}

const spatialPickPriority: Record<SpatialPickTargetKind, number> = {
	vertex: 0,
	edge: 1,
	face: 2,
	object: 3,
	surface: 4,
	part: 5,
};

function targetRayScore(ray: THREE.Ray, target: SpatialPickTarget): number | null {
	const points = target.points?.length ? target.points : [target.point];
	const box = new THREE.Box3();
	for (const point of points) box.expandByPoint(new THREE.Vector3(point[0], point[1], point[2]));
	box.expandByScalar(target.kind === "vertex" ? 0.12 : 0.08);
	const hit = ray.intersectBox(box, new THREE.Vector3());
	if (!hit) return null;
	return ray.origin.distanceTo(hit) + spatialPickPriority[target.kind] * 1e-4;
}

function pointerModifiersFromNativeEvent(event: PointerEvent): InteractionEvent["modifiers"] {
	return {
		alt: event.altKey,
		ctrl: event.ctrlKey,
		meta: event.metaKey,
		shift: event.shiftKey,
	};
}

function spatialSelectionModeFromModifiers(
	modifiers: { readonly alt?: boolean; readonly ctrl?: boolean; readonly meta?: boolean; readonly shift?: boolean } = {},
): SpatialSelectionMode {
	if (modifiers.shift && modifiers.ctrl) return "invertive";
	if (modifiers.shift) return "additive";
	if (modifiers.ctrl) return "subtractive";
	return "default";
}

function uniqueSelectionTargets(targets: readonly SelectionTarget[]): SelectionTarget[] {
	const out: SelectionTarget[] = [];
	const seen = new Set<string>();
	for (const target of targets) {
		const key = spatialSelectionTargetKey(target);
		if (seen.has(key)) continue;
		seen.add(key);
		out.push(target);
	}
	return out;
}

function mergeSelectionTargets(
	current: readonly SelectionTarget[],
	next: readonly SelectionTarget[],
	mode: SpatialSelectionMode,
): SelectionTarget[] {
	const uniqueNext = uniqueSelectionTargets(next);
	const nextKeys = new Set(uniqueNext.map(spatialSelectionTargetKey));
	if (mode === "default") return uniqueNext;
	if (mode === "additive") {
		const seen = new Set(current.map(spatialSelectionTargetKey));
		const merged = [...current];
		for (const target of uniqueNext) {
			const key = spatialSelectionTargetKey(target);
			if (seen.has(key)) continue;
			seen.add(key);
			merged.push(target);
		}
		return merged;
	}
	if (mode === "subtractive") return current.filter((target) => !nextKeys.has(spatialSelectionTargetKey(target)));
	const currentKeys = new Set(current.map(spatialSelectionTargetKey));
	return [
		...current.filter((target) => !nextKeys.has(spatialSelectionTargetKey(target))),
		...uniqueNext.filter((target) => !currentKeys.has(spatialSelectionTargetKey(target))),
	];
}

function dragDistance(a: { readonly x: number; readonly y: number }, b: { readonly x: number; readonly y: number }): number {
	return Math.hypot(b.x - a.x, b.y - a.y);
}

function spatialSelectionCoverageFromPath(path: readonly { readonly x: number; readonly y: number }[]): SpatialSelectionCoverage {
	const start = path[0];
	if (!start) return "full";
	for (const point of path.slice(1)) {
		const dx = point.x - start.x;
		if (Math.abs(dx) < 2) continue;
		return dx < 0 ? "partial" : "full";
	}
	const end = path[path.length - 1] ?? start;
	return end.x < start.x ? "partial" : "full";
}

function pointInRectangle(
	point: { readonly x: number; readonly y: number },
	rect: { readonly left: number; readonly right: number; readonly top: number; readonly bottom: number },
): boolean {
	return point.x >= rect.left && point.x <= rect.right && point.y >= rect.top && point.y <= rect.bottom;
}

function pointInPolygon(point: { readonly x: number; readonly y: number }, polygon: readonly { readonly x: number; readonly y: number }[]): boolean {
	let inside = false;
	for (let i = 0, j = polygon.length - 1; i < polygon.length; j = i++) {
		const a = polygon[i]!;
		const b = polygon[j]!;
		const intersects = a.y > point.y !== b.y > point.y && point.x < ((b.x - a.x) * (point.y - a.y)) / ((b.y - a.y) || 1e-9) + a.x;
		if (intersects) inside = !inside;
	}
	return inside;
}

function projectPointToClient(point: Vec3, camera: THREE.Camera, rect: DOMRect): { readonly x: number; readonly y: number } | null {
	const projected = new THREE.Vector3(point[0], point[1], point[2]).project(camera);
	if (!Number.isFinite(projected.x) || !Number.isFinite(projected.y) || !Number.isFinite(projected.z)) return null;
	if (projected.z < -1 || projected.z > 1) return null;
	return {
		x: rect.left + ((projected.x + 1) / 2) * rect.width,
		y: rect.top + ((1 - projected.y) / 2) * rect.height,
	};
}

function spatialPickTargetsFromClientPoint(
	client: { readonly x: number; readonly y: number },
	camera: THREE.Camera,
	rect: DOMRect,
	targets: readonly SpatialPickTarget[],
	selectionAccept: readonly ModelEntityKind[],
	kindToggles: SpatialPickKindToggles,
): SpatialPickTarget[] {
	const pointer = new THREE.Vector2(((client.x - rect.left) / rect.width) * 2 - 1, -(((client.y - rect.top) / rect.height) * 2 - 1));
	const raycaster = new THREE.Raycaster();
	raycaster.setFromCamera(pointer, camera);
	return spatialPickTargetsFromRay(raycaster.ray, targets, selectionAccept, kindToggles);
}

function spatialPickTargetsFromScreenSelection(
	drag: SpatialDragSelectionState,
	targets: readonly SpatialPickTarget[],
	camera: THREE.Camera,
	rect: DOMRect,
	selectionAccept: readonly ModelEntityKind[],
	kindToggles: SpatialPickKindToggles,
	geometryPreviewTransform?: ((point: Vec3) => Vec3) | null,
): SpatialPickTarget[] {
	const selectable = filterSpatialPickTargets(targets, selectionAccept, kindToggles);
	const mapPoint = geometryPreviewTransform ?? ((point: Vec3) => point);
	const rectBounds = {
		left: Math.min(drag.startClient.x, drag.currentClient.x),
		right: Math.max(drag.startClient.x, drag.currentClient.x),
		top: Math.min(drag.startClient.y, drag.currentClient.y),
		bottom: Math.max(drag.startClient.y, drag.currentClient.y),
	};
	const contains =
		drag.method === "rectangle"
			? (point: { readonly x: number; readonly y: number }) => pointInRectangle(point, rectBounds)
			: (point: { readonly x: number; readonly y: number }) => pointInPolygon(point, drag.path);
	return selectable.filter((target) => {
		const points = (target.points?.length ? target.points : [target.point]).map(mapPoint);
		const projected = points
			.map((point) => projectPointToClient(point, camera, rect))
			.filter((point): point is { readonly x: number; readonly y: number } => point !== null);
		if (projected.length === 0) return false;
		return drag.coverage === "partial" ? projected.some(contains) : projected.every(contains);
	});
}

function spatialPickTargetsFromRay(
	ray: THREE.Ray,
	targets: readonly SpatialPickTarget[],
	selectionAccept: readonly ModelEntityKind[],
	kindToggles: SpatialPickKindToggles,
): SpatialPickTarget[] {
	return filterSpatialPickTargets(targets, selectionAccept, kindToggles)
		.map((target) => ({ target, score: targetRayScore(ray, target) }))
		.filter((hit): hit is { readonly target: SpatialPickTarget; readonly score: number } => hit.score !== null)
		.sort((a, b) => a.score - b.score)
		.map((hit) => hit.target);
}

function targetStyle(target: SpatialPickTarget, hovered: boolean, selected: boolean): { color: string; emissive: string; opacity: number; lineWidth: number } {
	if (selected) return { color: "#ff77bb", emissive: "#551233", opacity: target.kind === "vertex" ? 1 : 0.34, lineWidth: 9 };
	if (hovered) return { color: "#66e8ff", emissive: "#003844", opacity: target.kind === "vertex" ? 1 : 0.28, lineWidth: 8 };
	if (target.kind === "vertex") return { color: "#ffdf7a", emissive: "#4a3000", opacity: 1, lineWidth: 5 };
	if (target.kind === "edge") return { color: "#ffd166", emissive: "#4a3000", opacity: 0.8, lineWidth: 5 };
	if (target.kind === "object" && !target.geometryKind) return { color: "#8ad4ff", emissive: "#103850", opacity: 0.28, lineWidth: 7 };
	return { color: "#f6c85f", emissive: "#332100", opacity: 0.16, lineWidth: 5 };
}

function selectionTargetPickKind(target: SelectionTarget): SpatialPickTargetKind | null {
	if (target.kind === "object") return "object";
	return GEOMETRY_KIND_TO_OBJECT_PICK[target.kind] ?? null;
}

function pinnedPickTargetKeys(keys: ReadonlySet<string>): ReadonlySet<string> {
	const out = new Set<string>();
	for (const key of keys) {
		out.add(key);
		const colon = key.indexOf(":");
		if (colon < 0) continue;
		const kind = key.slice(0, colon);
		const id = key.slice(colon + 1);
		const mapped = GEOMETRY_KIND_TO_OBJECT_PICK[kind as ModelEntityKind];
		if (mapped) out.add(`${mapped}:${id}`);
	}
	return out;
}

function spatialSelectionTarget(target: SpatialPickTarget): SelectionTarget {
	if (target.kind === "object" && !target.geometryKind) {
		return { kind: "object", id: target.id, editable: false };
	}
	const geometryKind = kernelGeometryKindForObjectPick(target.kind as SpatialGeometryPickTargetKind, target.geometryKind);
	return { kind: geometryKind, id: target.id, editable: true };
}

/** @emoji 🎯 Host geometry picking stays on without a bound interaction or once the session reaches a final state. */
export function replHostGeometryPickingEnabled(interactionId: string, spec: InteractionSpec, state: string): boolean {
	return !interactionId || !isInteractionSessionActive(spec, state);
}

/** @emoji 🖱️ Returns the closest pick target eligible for hover highlighting along a ray. */
export function pickHoverTargetFromRay(
	ray: THREE.Ray,
	targets: readonly SpatialPickTarget[],
	hoverKindToggles: SpatialPickKindToggles = {},
): SpatialPickTarget | null {
	return spatialPickTargetsFromRay(ray, targets, [], hoverKindToggles)[0] ?? null;
}

/** @emoji 📌 Keeps hover/selection overlays visible even when the normal visibility filter hides their kind. */
export function resolveSpatialPickTargetsToRender(
	viewTargets: readonly SpatialPickTarget[],
	filterKindToggles: SpatialPickKindToggles = {},
	pinnedTargetKeys: ReadonlySet<string> = new Set(),
): SpatialPickTarget[] {
	const pinnedKeys = pinnedPickTargetKeys(pinnedTargetKeys);
	const enabledTargets = filterSpatialPickTargetsForVisibility(viewTargets, filterKindToggles);
	const out = enabledTargets.filter((target) => {
		const key = spatialPickTargetKey(target);
		if (pinnedKeys.has(key)) return true;
		return target.kind === "vertex";
	});
	const seen = new Set(out.map(spatialPickTargetKey));
	for (const target of viewTargets) {
		const key = spatialPickTargetKey(target);
		if (!pinnedKeys.has(key) || seen.has(key)) continue;
		out.push(target);
		seen.add(key);
	}
	return out;
}

/** @emoji 👁️ Visual-only pick-target highlight; hit-testing is handled by `SpatialPickRayCatcher`. */
function SpatialPickTargetNode({
	target,
	geometryPreviewTransform = null,
	hoveredTargetKey,
	selectedTargetKey,
	selectedTargetKeys,
}: {
	readonly target: SpatialPickTarget;
	readonly geometryPreviewTransform?: ((point: Vec3) => Vec3) | null;
	readonly hoveredTargetKey?: string | null;
	readonly selectedTargetKey?: string | null;
	readonly selectedTargetKeys?: ReadonlySet<string> | null;
}): ReactNode {
	const mapPt = geometryPreviewTransform ?? ((p: Vec3) => p);
	const displayPoint = mapPt(target.point);
	const displayPoints = target.points?.map(mapPt);
	const targetKey = spatialPickTargetKey(target);
	const hovered = hoveredTargetKey === targetKey;
	const selected = selectedTargetKeys?.has(targetKey) ?? selectedTargetKey === targetKey;
	const style = targetStyle(target, hovered, selected);
	const userData = { spatialPickKey: targetKey };
	if (target.kind === "vertex") {
		return (
			<mesh position={displayPoint} userData={userData} raycast={raycastNone} renderOrder={8}>
				<sphereGeometry args={[selected || hovered ? 0.12 : 0.085, 16, 16]} />
				<meshStandardMaterial
					color={style.color}
					emissive={style.emissive}
					emissiveIntensity={0.45}
					depthTest={false}
					transparent
				/>
			</mesh>
		);
	}
	if (displayPoints && displayPoints.length >= 2 && target.kind === "edge") {
		return (
			<Line
				userData={userData}
				raycast={raycastNone}
				points={displayPoints.map((p) => [p[0], p[1], p[2]])}
				color={style.color}
				lineWidth={style.lineWidth}
			/>
		);
	}
	const bounds = displayPoints ? targetBounds(displayPoints) : null;
	if (!bounds) return null;
	return (
		<mesh position={bounds.center} scale={bounds.size} userData={userData} raycast={raycastNone} renderOrder={1}>
			<boxGeometry args={[1, 1, 1]} />
			<meshStandardMaterial
				color={style.color}
				emissive={style.emissive}
				emissiveIntensity={hovered || selected ? 0.35 : 0.08}
				transparent
				opacity={style.opacity}
				depthWrite={false}
				side={THREE.DoubleSide}
			/>
		</mesh>
	);
}

/** @emoji 🧵 Draws all geometry edges for imported factory geometry (one batched `lineSegments`). */
function GeometryFactoryWireframeLayer({
	geometry,
	visible = true,
}: {
	readonly geometry?: SpatialPickGeometry | null;
	readonly visible?: boolean;
}): ReactNode {
	const segments = useMemo(() => {
		if (!geometry) return [] as readonly (readonly [Vec3, Vec3])[];
		return collectGeometryEdgeSegments(geometryBuckets(geometry));
	}, [geometry]);
	const edgeGeometry = useMemo(() => {
		if (!segments.length) return null;
		const pos = new Float32Array(segments.length * 6);
		for (let i = 0; i < segments.length; i++) {
			const [a, b] = segments[i]!;
			const o = i * 6;
			pos[o] = a[0];
			pos[o + 1] = a[1];
			pos[o + 2] = a[2];
			pos[o + 3] = b[0];
			pos[o + 4] = b[1];
			pos[o + 5] = b[2];
		}
		const geo = new THREE.BufferGeometry();
		geo.setAttribute("position", new THREE.BufferAttribute(pos, 3));
		return geo;
	}, [segments]);
	useEffect(() => () => edgeGeometry?.dispose(), [edgeGeometry]);
	if (!visible || !edgeGeometry) return null;
	return (
		<lineSegments geometry={edgeGeometry} raycast={raycastNone} renderOrder={0}>
			<lineBasicMaterial color="#b8c8e8" transparent opacity={0.72} depthTest />
		</lineSegments>
	);
}

/** @emoji 🧲 Renders optional factory geometry as pickable snap/select targets. */
//#region 🧲SpatialPickGeometryLayer
export function SpatialPickGeometryLayer({
	geometry,
	activeModelDefinitionId = GEOMETRY_MODEL_DEFINITION_ID,
	modelDefinitionRevision = 0,
	geometryPreviewTransform = null,
	filterKindToggles = {},
	hoveredTargetKey,
	selectedTargetKey,
	selectedTargetKeys,
}: {
	readonly geometry?: SpatialPickGeometry | null;
	readonly activeModelDefinitionId?: string | null;
	readonly modelDefinitionRevision?: number;
	readonly geometryPreviewTransform?: ((point: Vec3) => Vec3) | null;
	readonly selectionAccept?: readonly ModelEntityKind[];
	/** @emoji 👁️ Which kinds are drawn as pick-target highlights (independent of selection). */
	readonly filterKindToggles?: SpatialPickKindToggles;
	readonly hoveredTargetKey?: string | null;
	readonly selectedTargetKey?: string | null;
	readonly selectedTargetKeys?: ReadonlySet<string> | null;
}): ReactNode {
	const topoRevision =
		geometry && typeof geometry === "object" && "revision" in geometry
			? Number((geometry as { revision?: unknown }).revision)
			: 0;
	const targets = useMemo(
		() => createSpatialPickTargets(geometry, activeModelDefinitionId),
		[geometry, topoRevision, modelDefinitionRevision, activeModelDefinitionId],
	);
	const viewTargets = useMemo(() => filterSpatialPickTargetsForActiveView(targets, activeModelDefinitionId ?? null), [targets, activeModelDefinitionId]);
	const pinnedTargetKeys = useMemo(() => {
		const keys = new Set<string>();
		if (hoveredTargetKey) keys.add(hoveredTargetKey);
		if (selectedTargetKey) keys.add(selectedTargetKey);
		selectedTargetKeys?.forEach((key) => keys.add(key));
		return keys;
	}, [hoveredTargetKey, selectedTargetKey, selectedTargetKeys]);
	const renderedTargets = useMemo(() => {
		return resolveSpatialPickTargetsToRender(viewTargets, filterKindToggles, pinnedTargetKeys);
	}, [viewTargets, filterKindToggles, pinnedTargetKeys]);
	return (
		<group>
			{renderedTargets.map((target) => (
				<SpatialPickTargetNode
					key={`${target.kind}:${target.id}`}
					target={target}
					geometryPreviewTransform={geometryPreviewTransform}
					hoveredTargetKey={hoveredTargetKey}
					selectedTargetKey={selectedTargetKey}
					selectedTargetKeys={selectedTargetKeys}
				/>
			))}
		</group>
	);
}
// #endregion 🧲GeometryInteraction

// #region 🧊CommittedMesh
/** @emoji 🧊 Builds a Three.js `BufferGeometry` from a kernel `MeshTransfer` (face groups preserved). */
export function buildBufferGeometryFromMeshTransfer(data: MeshTransfer): THREE.BufferGeometry {
	const geo = new THREE.BufferGeometry();
	if (!isRenderableMeshTransfer(data)) return geo;
	geo.setAttribute("position", new THREE.Float32BufferAttribute(data.position, 3));
	geo.setAttribute("normal", new THREE.Float32BufferAttribute(data.normal, 3));
	geo.setIndex(new THREE.BufferAttribute(data.index, 1));
	for (const g of data.faceGroups) geo.addGroup(g.start, g.count, 0);
	return geo;
}

/** @emoji 🎯 Maps a picked triangle index to B-Rep `FaceInfo` via grouped buffer ranges. */
export function resolveFaceInfoFromTriangleIndex(
	mesh: MeshTransfer,
	triangleIndex: number | null | undefined,
): FaceInfo | null {
	if (triangleIndex === null || triangleIndex === undefined) return null;
	const group = findFaceGroupAt(mesh.faceGroups, triangleIndex);
	if (!group) return null;
	return mesh.faceInfos.find((info) => info.entityId === group.entityId) ?? null;
}

/** @emoji ➖ B-Rep edge overlay from `MeshTransfer.edges` (kernel `meshEdges`, not triangle edges). */
function CommittedEdgeOverlay({ data, visible = true }: { readonly data: MeshTransfer; readonly visible?: boolean }): ReactNode {
	const geometry = useMemo(() => {
		const geo = new THREE.BufferGeometry();
		geo.setAttribute("position", new THREE.BufferAttribute(data.edges, 3));
		return geo;
	}, [data.edges]);
	useEffect(() => () => geometry.dispose(), [geometry]);
	if (!visible) return null;
	return (
		<lineSegments geometry={geometry} raycast={raycastNone}>
			<lineBasicMaterial color="#000000" depthTest />
		</lineSegments>
	);
}

export interface TessellatedCommitMeshProps {
	readonly mesh: MeshTransfer;
	readonly pickable?: boolean;
	readonly showFaces?: boolean;
	readonly showEdges?: boolean;
	readonly onFacePointerMove?: (info: FaceInfo, event: ThreeEvent<PointerEvent>) => void;
	readonly onFacePointerDown?: (info: FaceInfo, event: ThreeEvent<PointerEvent>) => void;
}

export const COMMITTED_MESH_FACE_OPACITY = 0.72;

/** @emoji 🧊 Shaded B-Rep mesh + edge overlay; optional face picking via `faceIndex`. */
export function TessellatedCommitMesh({
	mesh: data,
	pickable = false,
	showFaces = true,
	showEdges = true,
	onFacePointerMove,
	onFacePointerDown,
}: TessellatedCommitMeshProps): ReactNode {
	const geometry = useMemo(
		() => buildBufferGeometryFromMeshTransfer(data),
		[data.position, data.normal, data.index, data.faceGroups],
	);
	useEffect(() => () => geometry.dispose(), [geometry]);
	if (!showFaces && !showEdges) return null;
	const faceInfoById = useMemo(() => {
		const map = new Map<string, FaceInfo>();
		for (const info of data.faceInfos) map.set(String(info.entityId), info);
		return map;
	}, [data.faceInfos]);
	const resolveFace = useCallback(
		(event: ThreeEvent<PointerEvent>) => {
			const group = findFaceGroupAt(data.faceGroups, event.faceIndex ?? -1);
			if (!group) return null;
			return faceInfoById.get(String(group.entityId)) ?? null;
		},
		[data.faceGroups, faceInfoById],
	);
	const meshRaycast = pickable ? undefined : raycastNone;
	return (
		<group>
			{showFaces ? (
				<mesh
					geometry={geometry}
					raycast={meshRaycast}
					onPointerMove={
						pickable && onFacePointerMove
							? (e) => {
									const info = resolveFace(e);
									if (info) onFacePointerMove(info, e);
							  }
							: undefined
					}
					onPointerDown={
						pickable && onFacePointerDown
							? (e) => {
									const info = resolveFace(e);
									if (!info) return;
									e.stopPropagation();
									onFacePointerDown(info, e);
							  }
							: undefined
					}
				>
					<meshStandardMaterial
						color={data.color ?? "#9ad1ff"}
						metalness={0}
						roughness={0.45}
						emissive={data.color ?? "#9ad1ff"}
						emissiveIntensity={0.08}
						side={THREE.DoubleSide}
						polygonOffset
						polygonOffsetFactor={1}
						polygonOffsetUnits={1}
						transparent
						opacity={COMMITTED_MESH_FACE_OPACITY}
						depthWrite={false}
					/>
				</mesh>
			) : null}
			{data.edges.length > 0 ? <CommittedEdgeOverlay data={data} visible={showEdges} /> : null}
		</group>
	);
}

/** @emoji 🧊 Renders all committed document solids tessellated by the active kernel. */
export function CommittedMeshLayer({
	meshes,
	pickable = false,
	showFaces = true,
	showEdges = true,
	onFacePointerMove,
	onFacePointerDown,
}: {
	readonly meshes: readonly { readonly solid: SolidRef; readonly mesh: MeshTransfer }[];
	readonly pickable?: boolean;
	readonly showFaces?: boolean;
	readonly showEdges?: boolean;
	readonly onFacePointerMove?: (info: FaceInfo, event: ThreeEvent<PointerEvent>) => void;
	readonly onFacePointerDown?: (info: FaceInfo, event: ThreeEvent<PointerEvent>) => void;
}): ReactNode {
	if (meshes.length === 0 || (!showFaces && !showEdges)) return null;
	return (
		<group>
			{meshes.map((row, i) => (
				<TessellatedCommitMesh
					key={`${row.solid}:${meshTransferContentKey(row.mesh, i)}`}
					mesh={row.mesh}
					pickable={pickable}
					showFaces={showFaces}
					showEdges={showEdges}
					onFacePointerMove={onFacePointerMove}
					onFacePointerDown={onFacePointerDown}
				/>
			))}
		</group>
	);
}
// #endregion 🧊CommittedMesh

// #region 🪝Hooks
/** @emoji 🪝 Memoized `createInteractionRuntime` for React hosts. */
export function useInteractionRuntime(spec: InteractionSpec, opts: InteractionRuntimeOptions): InteractionRuntime {
	return useMemo(() => createInteractionRuntime(spec, opts), [spec, opts]);
}

/** @emoji 🪝 Subscribes to `InteractionRuntime` revision updates for React hosts. */
export function useInteractionSnapshot(rt: InteractionRuntime): InteractionSnapshot {
	return useSyncExternalStore(
		(cb) => rt.subscribe(cb),
		() => rt.getSnapshot(),
		() => rt.getSnapshot(),
	);
}

/** @emoji 🎛️ Resolves functional or literal host-state updates (testable without React). */
export function resolveHostStateNext<T>(value: T, next: T | ((prev: T) => T)): T {
	return typeof next === "function" ? (next as (prev: T) => T)(value) : next;
}

/** @emoji 🎛️ Controlled-or-uncontrolled state slice for embeddable spatial hosts. */
export function useHostState<T>(
	controlled: T | undefined,
	onChange: ((value: T) => void) | undefined,
	initial: T | (() => T),
): readonly [T, (next: T | ((prev: T) => T)) => void] {
	const [internal, setInternal] = useState(initial);
	const isControlled = controlled !== undefined;
	const value = isControlled ? controlled : internal;
	const setValue = useCallback(
		(next: T | ((prev: T) => T)) => {
			if (isControlled) {
				const resolved = resolveHostStateNext(controlled as T, next);
				onChange?.(resolved);
				return;
			}
			setInternal((prev) => {
				const resolved = resolveHostStateNext(prev, next);
				onChange?.(resolved);
				return resolved;
			});
		},
		[controlled, isControlled, onChange],
	);
	return [value, setValue] as const;
}
// #endregion 🪝Hooks

// #region 🪩Canvas
export interface InteractionCanvasProps {
	readonly children: ReactNode;
	readonly onCanvasReady?: (binding: { readonly camera: THREE.Camera; readonly domElement: HTMLCanvasElement }) => void;
	/** @emoji 🎞️ `always` while an interaction session runs; `demand` when idle for GPU savings. */
	readonly frameloop?: "always" | "demand";
	readonly background?: string;
	readonly cameraPosition?: Vec3;
	readonly cameraFov?: number;
	readonly cameraNear?: number;
	readonly cameraFar?: number;
	readonly dpr?: number | [number, number];
	readonly shadows?: boolean | "basic" | "percentage" | "soft" | "variance";
	readonly style?: CSSProperties;
	readonly className?: string;
	readonly gl?: React.ComponentProps<typeof Canvas>["gl"];
	readonly onPointerDown?: (event: PointerEvent) => void;
	readonly onPointerMove?: (event: PointerEvent) => void;
	readonly onPointerUp?: (event: PointerEvent) => void;
	readonly onPointerLeave?: (event: PointerEvent) => void;
	readonly onPointerCancel?: (event: PointerEvent) => void;
	readonly onWheel?: (event: WheelEvent) => void;
	readonly onContextMenu?: (event: MouseEvent) => void;
	readonly onDoubleClick?: (event: MouseEvent) => void;
	readonly onLostPointerCapture?: (event: PointerEvent) => void;
}

/** @emoji 📡 Host event callbacks accepted by {@link InteractionCanvas}. */
export type InteractionCanvasHostCallbacks = Pick<
	InteractionCanvasProps,
	| "onCanvasReady"
	| "onPointerDown"
	| "onPointerMove"
	| "onPointerUp"
	| "onPointerLeave"
	| "onPointerCancel"
	| "onWheel"
	| "onContextMenu"
	| "onDoubleClick"
	| "onLostPointerCapture"
>;

export type SpatialAutoFitBehavior = "initial" | "changes";

export function spatialAutoFitShouldRun(
	behavior: SpatialAutoFitBehavior,
	key: string,
	lastKey: string,
	hasApplied: boolean,
): boolean {
	if (!key || key === lastKey) return false;
	return behavior === "changes" || !hasApplied;
}

/** @emoji 🛰️ Frames the camera to fit committed meshes and/or factory geometry (playground auto-fit). */
export function SpatialAutoFit({
	meshes,
	geometry = null,
	padding = 1.25,
	behavior = "initial",
}: {
	readonly meshes: readonly MeshTransfer[];
	readonly geometry?: SpatialPickGeometry | null;
	readonly padding?: number;
	readonly behavior?: SpatialAutoFitBehavior;
}): null {
	const { camera, controls, invalidate } = useThree();
	const geometryRevision =
		geometry && typeof geometry === "object" && "revision" in geometry
			? Number((geometry as { revision?: unknown }).revision)
			: 0;
	const bounds = useMemo(
		() => mergeSpatialSceneBounds(boundsFromMeshTransfers(meshes), boundsFromSpatialPickGeometry(geometry)),
		[meshes, geometry, geometryRevision],
	);
	const lastKey = useRef("");
	const hasApplied = useRef(false);
	useEffect(() => {
		if (!bounds) return;
		const meshKey = meshes.map((m, i) => meshTransferContentKey(m, i)).join("|");
		const key = `${geometryRevision}:${meshKey}`;
		if (!spatialAutoFitShouldRun(behavior, key, lastKey.current, hasApplied.current)) return;
		lastKey.current = key;
		hasApplied.current = true;
		applySpatialAutoFitCamera(camera, bounds, padding, controls);
		invalidate();
	}, [behavior, bounds, camera, controls, geometryRevision, invalidate, meshes, padding]);
	return null;
}

export function applySpatialAutoFitCamera(
	camera: THREE.Camera,
	bounds: { readonly center: Vec3; readonly radius: number },
	padding = 1.25,
	controls?: unknown,
): void {
	const [cx, cy, cz] = bounds.center;
	const dist = Math.max(bounds.radius * padding, 2);
	camera.position.set(cx + dist, cy + dist, cz + dist * 0.85);
	const orbit = controls as { readonly target?: THREE.Vector3; update?: () => void } | undefined;
	if (orbit?.target) {
		orbit.target.set(cx, cy, cz);
		orbit.update?.();
	} else {
		camera.lookAt(cx, cy, cz);
	}
	if ("updateProjectionMatrix" in camera && typeof camera.updateProjectionMatrix === "function") {
		camera.updateProjectionMatrix();
	}
}

/** @emoji 🔄 Invalidates demand frameloop when host-driven scene visuals change. */
function InvalidateOnRevision({ revision }: { readonly revision: string | number }): null {
	const invalidate = useThree((state) => state.invalidate);
	useEffect(() => {
		invalidate();
	}, [revision, invalidate]);
	return null;
}

/** @emoji 🔄 Keeps demand frameloop alive while the camera moves (playground `Invalidator`). */
function SpatialInvalidator(): null {
	const { controls, camera } = useThree();
	const lastPos = useRef(new THREE.Vector3());
	const lastTarget = useRef(new THREE.Vector3());
	useFrame(({ invalidate }) => {
		// eslint-disable-next-line @typescript-eslint/no-explicit-any -- drei OrbitControls
		const ctrl = controls as any;
		if (!ctrl) return;
		const target = ctrl.target as THREE.Vector3 | undefined;
		const moved =
			!camera.position.equals(lastPos.current) || (target ? !target.equals(lastTarget.current) : false);
		if (moved) {
			lastPos.current.copy(camera.position);
			if (target) lastTarget.current.copy(target);
			invalidate();
		}
	});
	return null;
}

/** @emoji 🛰️ Orbit controls that repaint on demand and never block R3F pointer routing. */
function SpatialOrbitControls({
	onCameraNavigate,
}: {
	readonly onCameraNavigate?: (active: boolean) => void;
}): ReactNode {
	const invalidate = useThree((state) => state.invalidate);
	return (
		<OrbitControls
			makeDefault
			enableDamping={false}
			onChange={() => invalidate()}
			onStart={() => onCameraNavigate?.(true)}
			onEnd={() => onCameraNavigate?.(false)}
			mouseButtons={{
				LEFT: -1 as unknown as MOUSE,
				MIDDLE: MOUSE.DOLLY,
				RIGHT: MOUSE.ROTATE,
			}}
		/>
	);
}

/** @emoji 🪩 Root `<Canvas>` configuration for factory viewports. */
export function InteractionCanvas({
	children,
	onCanvasReady,
	frameloop = "demand",
	background = defaultInteractionSpatialViewTheme.background,
	cameraPosition = [10, 10, 8],
	cameraFov = 45,
	cameraNear,
	cameraFar,
	dpr,
	shadows,
	style,
	className,
	gl,
	onPointerDown,
	onPointerMove,
	onPointerUp,
	onPointerLeave,
	onPointerCancel,
	onWheel,
	onContextMenu,
	onDoubleClick,
	onLostPointerCapture,
}: InteractionCanvasProps): ReactNode {
	return (
		<Canvas
			frameloop={frameloop}
			className={className}
			style={{ height: "100%", width: "100%", ...style }}
			dpr={dpr}
			shadows={shadows}
			camera={{
				up: [0, 0, 1],
				position: cameraPosition,
				fov: cameraFov,
				...(cameraNear !== undefined ? { near: cameraNear } : {}),
				...(cameraFar !== undefined ? { far: cameraFar } : {}),
			}}
			gl={gl}
			onPointerDown={(event) => onPointerDown?.(event.nativeEvent)}
			onPointerMove={(event) => onPointerMove?.(event.nativeEvent)}
			onPointerUp={(event) => onPointerUp?.(event.nativeEvent)}
			onPointerLeave={(event) => onPointerLeave?.(event.nativeEvent)}
			onPointerCancel={(event) => onPointerCancel?.(event.nativeEvent)}
			onWheel={(event) => onWheel?.(event.nativeEvent)}
			onContextMenu={(event) => onContextMenu?.(event.nativeEvent)}
			onDoubleClick={(event) => onDoubleClick?.(event.nativeEvent)}
			onLostPointerCapture={(event) => onLostPointerCapture?.(event.nativeEvent)}
			onCreated={({ camera, gl: renderer }) => onCanvasReady?.({ camera, domElement: renderer.domElement })}
		>
			<color attach="background" args={[background ?? "#080810"]} />
			{children}
		</Canvas>
	);
}

export interface InteractionSpatialViewProps {
	readonly previewKernel?: SpatialPreviewKernel;
	readonly snapshot: InteractionSnapshot;
	readonly onGroundPick?: (point: Vec3, event: InteractionEvent) => void;
	/** @emoji 🖱️ `pointer.move` hits ground (XY at fixed Z); height slab passes full 3D. */
	readonly onScenePointerMove?: (point: Vec3, event: InteractionEvent) => void;
	readonly onInteractionEvent?: (event: InteractionEvent) => void;
	readonly pickEnabled?: boolean;
	readonly committedMesh?: MeshTransfer | null;
	readonly committedMeshes?: readonly { readonly solid: SolidRef; readonly mesh: MeshTransfer }[];
	readonly geometry?: SpatialPickGeometry | null;
	/** @emoji 🧲 Pick-target source; defaults to `geometry` (use builtin geometry when the active model is typology-only). */
	readonly pickGeometry?: SpatialPickGeometry | null;
	readonly activeModelDefinitionId?: string | null;
	readonly modelDefinitionRevision?: number;
	/** @emoji 🖼️ When set, drives `InteractionDisplay` instead of `snapshot.display` (e.g. merged archived footprints). */
	readonly displayModel?: DisplayModel;
	readonly renderDisplayItem?: SpatialDisplayItemRenderer;
	readonly selectionAccept?: readonly ModelEntityKind[];
	readonly filterKindToggles?: SpatialPickKindToggles;
	readonly selectionKindToggles?: SpatialPickKindToggles;
	/** @emoji 🖱️ Hover raycast kind filter; defaults to `selectionKindToggles` when omitted. */
	readonly hoverKindToggles?: SpatialPickKindToggles;
	readonly hoveredTargetKey?: string | null;
	readonly selectedTargetKey?: string | null;
	readonly selectedTargetKeys?: ReadonlySet<string> | null;
	readonly hostSelectionEnabled?: boolean;
	readonly onSelectionRequest?: (request: SpatialSelectionRequest) => void;
	readonly onCameraNavigate?: (active: boolean) => void;
	readonly onCommittedFacePointerDown?: (info: FaceInfo, event: ThreeEvent<PointerEvent>) => void;
	readonly onCommittedFacePointerMove?: (info: FaceInfo, event: ThreeEvent<PointerEvent>) => void;
	readonly onSnapshotStateChange?: (state: string) => void;
	readonly onSnapshotRevisionChange?: (revision: number) => void;
	readonly onPickEnabledChange?: (enabled: boolean) => void;
	/** @emoji 🧲 When false, skips pick-target meshes (during active interaction sessions). */
	readonly showPickLayer?: boolean;
	readonly committedMeshPickable?: boolean;
	readonly autoFitMeshes?: boolean;
	readonly autoFitBehavior?: SpatialAutoFitBehavior;
	readonly theme?: InteractionSpatialViewTheme;
	readonly slots?: InteractionSpatialViewSlots;
}

/** @emoji 📡 Host event callbacks accepted by {@link InteractionSpatialView}. */
export type InteractionSpatialViewHostCallbacks = Pick<
	InteractionSpatialViewProps,
	| "onGroundPick"
	| "onScenePointerMove"
	| "onInteractionEvent"
	| "onSelectionRequest"
	| "onCameraNavigate"
	| "onCommittedFacePointerDown"
	| "onCommittedFacePointerMove"
	| "onSnapshotStateChange"
	| "onSnapshotRevisionChange"
	| "onPickEnabledChange"
>;

/** @emoji 🖱️ Ground-plane picking is command input and must stay independent from host geometry selection. */
export function interactionSpatialGroundPickPlaneEnabled(
	snapshot: Pick<InteractionSnapshot, "spatialInteraction" | "state">,
	pickEnabled: boolean,
	selectionAccept: readonly ModelEntityKind[],
): boolean {
	const si = snapshot.spatialInteraction;
	return pickEnabled !== false && si.spatialGroundPick && selectionAccept.length === 0 && !si.pickDisabledStates.includes(snapshot.state);
}

/** @emoji 🪩 Lights, orbit controls, ground picking, factory overlays, optional committed mesh. */
export function InteractionSpatialView({
	previewKernel = r3fPreviewKernel,
	snapshot,
	onGroundPick,
	onScenePointerMove,
	onInteractionEvent,
	pickEnabled = true,
	committedMesh,
	committedMeshes,
	geometry,
	pickGeometry: pickGeometryProp,
	activeModelDefinitionId = GEOMETRY_MODEL_DEFINITION_ID,
	modelDefinitionRevision = 0,
	displayModel,
	renderDisplayItem,
	selectionAccept = [],
	filterKindToggles = {},
	hoveredTargetKey,
	selectedTargetKey,
	selectedTargetKeys,
	onCameraNavigate,
	onCommittedFacePointerDown,
	onCommittedFacePointerMove,
	onSnapshotStateChange,
	onSnapshotRevisionChange,
	onPickEnabledChange,
	showPickLayer = true,
	committedMeshPickable = false,
	autoFitMeshes = false,
	autoFitBehavior = "initial",
	theme = defaultInteractionSpatialViewTheme,
	slots,
}: InteractionSpatialViewProps): ReactNode {
	useEffect(() => {
		bindScenePreviewKernel(previewKernel);
	}, [previewKernel]);
	useEffect(() => {
		onSnapshotStateChange?.(snapshot.state);
	}, [snapshot.state, onSnapshotStateChange]);
	useEffect(() => {
		onSnapshotRevisionChange?.(snapshot.revision);
	}, [snapshot.revision, onSnapshotRevisionChange]);
	const resolvedTheme = { ...defaultInteractionSpatialViewTheme, ...theme };
	const gridDivisions = resolvedTheme.gridDivisions ?? 40;
	const gridSize = resolvedTheme.gridSize ?? 40;
	const gridHelper = useMemo(() => {
		const g = new THREE.GridHelper(gridSize, gridDivisions, 0x3a3a55, 0x1c1c28);
		g.rotation.x = Math.PI / 2;
		g.position.set(0, 0, 0.002);
		g.traverse((obj) => {
			obj.raycast = raycastNone;
		});
		return g;
	}, [gridDivisions, gridSize]);
	const layerMeshes = useMemo(() => {
		if (committedMeshes?.length) return committedMeshes;
		if (committedMesh) return [{ solid: solidRef("committed"), mesh: committedMesh }];
		return [];
	}, [committedMeshes, committedMesh]);
	const autoFitSources = useMemo(() => layerMeshes.map((row) => row.mesh), [layerMeshes]);
	const ctx = snapshot.context;
	const geometryPreviewTransform = useMemo(
		() => geometryPreviewTransformFromDisplay(displayModel ?? snapshot.display),
		[displayModel, snapshot.display],
	);
	const origin = vec3FromSnapshotContext(ctx, "origin");
	const corner = vec3FromSnapshotContext(ctx, "corner");
	const si = snapshot.spatialInteraction;
	const groundMoveOn =
		si.spatialGroundPick && si.groundPointerMoveStates.includes(snapshot.state) && Boolean(onScenePointerMove);
	const heightMoveOn =
		si.spatialGroundPick &&
		si.heightDragStates.includes(snapshot.state) &&
		Boolean(onScenePointerMove) &&
		origin !== null &&
		corner !== null;
	const zRodMoveOn =
		si.spatialGroundPick &&
		si.verticalRodStates.includes(snapshot.state) &&
		Boolean(onScenePointerMove) &&
		origin !== null;
	const pickPlaneEnabled = interactionSpatialGroundPickPlaneEnabled(snapshot, pickEnabled, selectionAccept);
	useEffect(() => {
		onPickEnabledChange?.(pickPlaneEnabled);
	}, [pickPlaneEnabled, onPickEnabledChange]);
	const onGroundPickEvent = (point: Vec3) => {
		const event = createSpatialPickEvent("pointer.down", point, null);
		onInteractionEvent?.(event);
		onGroundPick?.(point, event);
	};
	const onGroundContextEvent = (point: Vec3) => {
		onInteractionEvent?.({ kind: "contextmenu", point, modifiers: {} });
	};
	const onScenePointerMoveEvent = (point: Vec3) => {
		const event = createSpatialPickEvent("pointer.move", point, null);
		onInteractionEvent?.(event);
		onScenePointerMove?.(point, event);
	};
	const dirPos = resolvedTheme.directionalPosition ?? [12, 18, 10];
	const geometryRevision =
		geometry && typeof geometry === "object" && "revision" in geometry
			? Number((geometry as { revision?: unknown }).revision)
			: 0;
	const sceneVisibility = useMemo(
		() => resolveSpatialSceneVisibility(activeModelDefinitionId, filterKindToggles),
		[activeModelDefinitionId, filterKindToggles],
	);
	const scenePickGeometry = pickGeometryProp ?? geometry;
	const pickGeometryRevision =
		scenePickGeometry && typeof scenePickGeometry === "object" && "revision" in scenePickGeometry
			? Number((scenePickGeometry as { revision?: unknown }).revision)
			: 0;
	return (
		<>
			{slots?.beforeScene}
			<InvalidateOnRevision
				revision={`${snapshot.revision}:${modelDefinitionRevision}:${geometryRevision}:${pickGeometryRevision}:${hoveredTargetKey ?? ""}`}
			/>
			<SpatialInvalidator />
			{autoFitMeshes ? <SpatialAutoFit meshes={autoFitSources} geometry={geometry} behavior={autoFitBehavior} /> : null}
			{slots?.environment}
			{slots?.lights ?? (
				<>
					<ambientLight intensity={resolvedTheme.ambientIntensity ?? 0.45} />
					<directionalLight position={dirPos} intensity={resolvedTheme.directionalIntensity ?? 1.1} />
				</>
			)}
			<SpatialOrbitControls onCameraNavigate={onCameraNavigate} />
			<primitive object={gridHelper} />
			<GroundPickPlane
				enabled={pickPlaneEnabled}
				onPick={onGroundPickEvent}
				onContextPick={onGroundContextEvent}
				onPointerMove={onScenePointerMoveEvent}
				pointerMoveEnabled={groundMoveOn}
				planeColor={resolvedTheme.groundPlaneColor}
				planeOpacity={resolvedTheme.groundPlaneOpacity}
			/>
			<GeometryFactoryWireframeLayer geometry={scenePickGeometry} visible={sceneVisibility.showFactoryWireframe} />
			{showPickLayer ? (
				<SpatialPickGeometryLayer
					geometry={scenePickGeometry}
					activeModelDefinitionId={activeModelDefinitionId}
					modelDefinitionRevision={modelDefinitionRevision}
					geometryPreviewTransform={geometryPreviewTransform}
					selectionAccept={selectionAccept}
					filterKindToggles={filterKindToggles}
					hoveredTargetKey={hoveredTargetKey}
					selectedTargetKey={selectedTargetKey}
					selectedTargetKeys={selectedTargetKeys}
				/>
			) : null}
			{heightMoveOn && origin && corner ? (
				<HeightDragSurface
					origin={origin}
					corner={corner}
					enabled={heightMoveOn}
					onPointerMove={onScenePointerMoveEvent}
				/>
			) : null}
			{zRodMoveOn && origin ? (
				<VerticalZDragRod origin={origin} enabled={zRodMoveOn} onPointerMove={onScenePointerMoveEvent} />
			) : null}
			<CommittedMeshLayer
				meshes={layerMeshes}
				pickable={committedMeshPickable}
				showFaces={sceneVisibility.showCommittedFaces}
				showEdges={sceneVisibility.showCommittedEdges}
				onFacePointerDown={onCommittedFacePointerDown}
				onFacePointerMove={onCommittedFacePointerMove}
			/>
			<InteractionDisplay
				geometry={geometry}
				model={displayModel ?? snapshot.display}
				renderItem={renderDisplayItem}
			/>
			{slots?.afterDisplay}
			{slots?.afterCommitted}
		</>
	);
}
// #endregion 🪩Canvas

// #region 🪩Repl
type ReplSuggestKind = "interaction" | "transition" | "action" | "selection";

interface ReplSuggestion {
	readonly kind: ReplSuggestKind;
	readonly key: string;
	readonly label: string;
	readonly detail: string;
	readonly transition?: InteractionKeybindRow;
	readonly interactionId?: string;
	readonly onRun: () => void;
}

function resolveScopedSpatialInteractionKey(token: string, modelDefinitionId: string): SpatialInteraction | null {
	return resolveSpatialInteractionKeyForModelDefinition(modelDefinitionId, token);
}

function replCommandTextWithoutSpaces(text: string): string {
	return text.replace(/\s+/g, "");
}

function replFirstWireId(model: Model): string | null {
	const ks = Object.keys(model.wires);
	return ks.length ? model.wires[ks[0]!]!.id : null;
}

function replFirstFaceId(model: Model): string | null {
	const ks = Object.keys(model.faces);
	return ks.length ? model.faces[ks[0]!]!.id : null;
}

function replBuildDispatchEvent(
	row: InteractionKeybindRow,
	opts: { readonly interactionId: string; readonly model: Model },
): InteractionEvent | null {
	const { interactionId, topo } = opts;
	if (row.eventKind === "set.height" || row.eventKind === "set.distance" || row.eventKind === "set.footprint") return null;
	if (row.eventKind === "selection.changed") {
		if (interactionId === "feature.extrudeWire") {
			const wid = replFirstWireId(model);
			if (!wid) return null;
			return { kind: "selection.changed", targets: [{ kind: "wire", id: wid, editable: true }], modifiers: {} };
		}
		if (interactionId === "feature.offsetSurface") {
			const fid = replFirstFaceId(model);
			if (!fid) return null;
			return { kind: "selection.changed", targets: [{ kind: "face", id: fid, editable: true }], modifiers: {} };
		}
		return null;
	}
	return { kind: row.eventKind, modifiers: {} };
}

function replTryParseValueInteraction(line: string, spec: InteractionSpec, state: string): InteractionEvent | null {
	const t = line.trim();
	const m = t.match(/^(\S+)\s+(.+)$/);
	if (!m) return null;
	const head = m[1]!.toLowerCase();
	const tail = m[2]!.trim();
	const rows = listKeyedInteractionTransitions(spec, state);
	for (const row of rows) {
		if (row.eventKind === "set.height") {
			if (head !== row.key.toLowerCase() && head !== "height") continue;
			const v = Number(tail);
			if (!Number.isFinite(v) || v <= 0) return null;
			return { kind: "set.height", value: v, modifiers: {} };
		}
		if (row.eventKind === "set.distance") {
			if (head !== row.key.toLowerCase() && head !== "dist" && head !== "distance") continue;
			const v = Number(tail);
			if (!Number.isFinite(v)) return null;
			return { kind: "set.distance", value: v, modifiers: {} };
		}
		if (row.eventKind === "set.footprint") {
			if (head !== row.key.toLowerCase() && head !== "footprint" && head !== "lw") continue;
			const parts = tail.split(/\s+/);
			const L = Number(parts[0]);
			const W = Number(parts[1]);
			if (!Number.isFinite(L) || !Number.isFinite(W)) return null;
			return { kind: "set.footprint", value: { length: L, width: W }, modifiers: {} };
		}
		if (row.eventKind.startsWith("set.")) {
			const alias = row.eventKind.slice("set.".length).toLowerCase();
			if (head !== row.key.toLowerCase() && head !== alias && head !== "number" && head !== "n") continue;
			const v = Number(tail);
			if (!Number.isFinite(v)) return null;
			return { kind: row.eventKind, value: v, modifiers: {} };
		}
	}
	return null;
}

function replSuggestionHaystack(s: ReplSuggestion): string {
	return `${s.key} ${s.label} ${s.detail}`.toLowerCase();
}

function replRankScore(query: string, s: ReplSuggestion): number {
	const ql = query.trim().toLowerCase();
	if (!ql) return -1;
	const key = s.key.toLowerCase();
	const label = s.label.toLowerCase();
	const detail = s.detail.toLowerCase();
	if (key.startsWith(ql)) return 4000 - key.length;
	if (label.startsWith(ql)) return 3000 - label.length;
	if (detail.startsWith(ql)) return 2000 - detail.length;
	if (replSuggestionHaystack(s).includes(ql)) return 1000;
	return -1;
}

export function replFilterSuggestions(query: string, all: readonly ReplSuggestion[]): ReplSuggestion[] {
	const q = query.trim();
	if (!q) return [];
	return all
		.map((s) => ({ s, score: replRankScore(q, s) }))
		.filter((row) => row.score >= 0)
		.sort((a, b) => b.score - a.score)
		.map((row) => row.s);
}

/** @emoji ⌨️ Inline completion suffix for the active suggestion (longest prefix match on key, label, or detail). */
export function replCompletionSuffix(query: string, suggestion: ReplSuggestion | undefined): string {
	if (!query.trim() || !suggestion) return "";
	const q = query;
	const ql = q.toLowerCase();
	let best = "";
	for (const text of [suggestion.label, suggestion.detail, suggestion.key]) {
		if (!text.toLowerCase().startsWith(ql)) continue;
		const suffix = text.slice(q.length);
		if (suffix.length > best.length) best = suffix;
	}
	return best;
}

/** @emoji ⌨️ First non-empty inline completion suffix across ranked matches. */
export function replActiveCompletionSuffix(query: string, matches: readonly ReplSuggestion[], index: number): string {
	if (!query.trim() || !matches.length) return "";
	const order = [matches[Math.min(index, matches.length - 1)]!, ...matches];
	const seen = new Set<ReplSuggestion>();
	for (const s of order) {
		if (seen.has(s)) continue;
		seen.add(s);
		const suffix = replCompletionSuffix(query, s);
		if (suffix) return suffix;
	}
	return "";
}

export function replPaletteRows(cmdLine: string, all: readonly ReplSuggestion[]): ReplSuggestion[] {
	return replFilterSuggestions(cmdLine, all);
}

function replInteractionSuggestions(query: string, all: readonly ReplSuggestion[]): ReplSuggestion[] {
	const xs = query.trim() ? replFilterSuggestions(query, all) : all;
	return xs.filter((suggestion) => suggestion.kind === "interaction");
}

function replExactInteractionSuggestion(query: string, all: readonly ReplSuggestion[]): ReplSuggestion | null {
	const raw = query.trim().toLowerCase();
	if (!raw) return null;
	for (const suggestion of all) {
		if (suggestion.kind !== "interaction") continue;
		for (const text of [suggestion.key, suggestion.label, suggestion.detail]) {
			if (text.toLowerCase() === raw) return suggestion;
		}
	}
	return null;
}

function replInteractionSuggestionOnSpace(query: string, matches: readonly ReplSuggestion[], all: readonly ReplSuggestion[]): ReplSuggestion | null {
	const exact = replExactInteractionSuggestion(query, all);
	if (exact) return exact;
	return matches.find((suggestion) => suggestion.kind === "interaction") ?? null;
}

function replInteractionIdOnSpace(
	query: string,
	matches: readonly ReplSuggestion[],
	all: readonly ReplSuggestion[],
	lastFinalizedInteractionId: string,
): string | null {
	if (!query.trim()) return lastFinalizedInteractionId || null;
	return replInteractionSuggestionOnSpace(query, matches, all)?.interactionId ?? null;
}

function replIsQueryTypingTarget(t: EventTarget | null): boolean {
	return t instanceof HTMLTextAreaElement;
}

function replShouldRepeatInteractionOnSpace(
	event: {
		readonly key: string;
		readonly ctrlKey: boolean;
		readonly metaKey: boolean;
		readonly altKey: boolean;
		readonly defaultPrevented: boolean;
		readonly isComposing: boolean;
		readonly target: EventTarget | null;
	},
	state: {
		readonly interactionActive: boolean;
		readonly cmdTarget: EventTarget | null;
	},
): boolean {
	if (event.defaultPrevented || event.isComposing || state.interactionActive) return false;
	if (event.key !== " " || event.ctrlKey || event.metaKey || event.altKey) return false;
	if (replIsQueryTypingTarget(event.target)) return false;
	return event.target !== state.cmdTarget;
}

function replEscapeAction(state: {
	readonly hasInteraction: boolean;
	readonly interactionActive: boolean;
	readonly cmdLine: string;
	readonly hasSelectionMenu: boolean;
}): "abort" | "dismiss" | "none" {
	if (state.hasInteraction || state.interactionActive) return "abort";
	if (state.cmdLine.trim() || state.hasSelectionMenu) return "dismiss";
	return "none";
}

function replSelectionEvent(selection: readonly SelectionTarget[], point?: Vec3): InteractionEvent {
	return point ? { kind: "selection.changed", targets: selection, point, modifiers: {} } : { kind: "selection.changed", targets: selection, modifiers: {} };
}

function replStartEvent(selection: readonly SelectionTarget[]): InteractionEvent {
	return { kind: "start", targets: selection, modifiers: {} };
}

function replSelectionAccepted(accept: readonly ModelEntityKind[], selection: readonly SelectionTarget[]): SelectionTarget[] {
	return selection.filter((target) => accept.includes(target.kind));
}

/** @emoji 🪪 Reads validated `context.targets` for interaction highlight sync. */
export function replInteractionSelectionFromContext(ctx: Record<string, unknown>): readonly SelectionTarget[] {
	const raw = ctx.targets;
	if (!Array.isArray(raw)) return [];
	return raw.filter((target): target is SelectionTarget => {
		return Boolean(
			target &&
			typeof target === "object" &&
			"kind" in target &&
			"id" in target &&
			typeof (target as { kind?: unknown }).kind === "string" &&
			typeof (target as { id?: unknown }).id === "string",
		);
	});
}

/** @emoji 🪪 Shallow equality for ordered selection target lists. */
export function replSelectionTargetsEqual(a: readonly SelectionTarget[], b: readonly SelectionTarget[]): boolean {
	if (a.length !== b.length) return false;
	for (let i = 0; i < a.length; i++) {
		const x = a[i]!;
		const y = b[i]!;
		if (x.kind !== y.kind || x.id !== y.id || x.editable !== y.editable) return false;
	}
	return true;
}

function replApplySelectionPick(
	current: readonly SelectionTarget[],
	picked: readonly SelectionTarget[],
	modifiers: InteractionEvent["modifiers"],
): SelectionTarget[] {
	const modeModifiers = (modifiers ?? {}) as { readonly alt?: boolean; readonly ctrl?: boolean; readonly meta?: boolean; readonly shift?: boolean };
	return mergeSelectionTargets(current, picked, spatialSelectionModeFromModifiers(modeModifiers));
}

function replSelectionLayerParts(
	interactionActive: boolean,
	activeModelDefinitionId: string | null,
	rendererSelection: readonly SelectionTarget[],
	interactionSelection: readonly SelectionTarget[],
): {
	readonly layer: readonly SelectionTarget[];
	readonly inView: readonly SelectionTarget[];
	readonly outOfView: readonly SelectionTarget[];
} {
	const layer = interactionActive ? interactionSelection : rendererSelection;
	const entityKinds = new Set(modelDefinitionSelectionEntityKinds(activeModelDefinitionId ?? GEOMETRY_MODEL_DEFINITION_ID));
	const inView: SelectionTarget[] = [];
	const outOfView: SelectionTarget[] = [];
	for (const target of layer) {
		if (target.kind === "object" && target.editable === false) {
			if (entityKinds.has("object")) inView.push(target);
			else outOfView.push(target);
			continue;
		}
		if (entityKinds.has(target.kind)) inView.push(target);
		else outOfView.push(target);
	}
	return { layer, inView, outOfView };
}

/** @emoji 🪪 Removes in-view targets of a pick kind when its selection toggle is turned off. */
export function replPruneSelectionByKind(
	selection: readonly SelectionTarget[],
	activeModelDefinitionId: string | null,
	kind: SpatialPickTargetKind,
): SelectionTarget[] {
	if (!spatialPickKindsForActiveView(activeModelDefinitionId).has(kind)) return [...selection];
	if (kind === "object") {
		return selection.filter((target) => target.kind !== "object" || target.editable !== false);
	}
	const geometryKinds: readonly ModelEntityKind[] =
		kind === "vertex"
			? ["vertex", "anchor"]
			: kind === "edge"
				? ["edge", "wire"]
				: kind === "face"
					? ["face", "shell"]
					: ["solid", "geometry"];
	return selection.filter((target) => !geometryKinds.includes(target.kind) && selectionTargetPickKind(target) !== kind);
}

/** @emoji 🪪 Picks the highlight layer: interaction picks while active, else renderer selection, scoped to active model definition. */
export function replDisplayedSelectionTargets(
	interactionActive: boolean,
	activeModelDefinitionId: string | null,
	rendererSelection: readonly SelectionTarget[],
	interactionSelection: readonly SelectionTarget[],
): readonly SelectionTarget[] {
	return replSelectionLayerParts(interactionActive, activeModelDefinitionId, rendererSelection, interactionSelection).inView;
}

/** @emoji 🪪 Merges a pick into the active selection layer without touching out-of-view targets. */
export function replMergeSelectionPickInView(
	interactionActive: boolean,
	activeModelDefinitionId: string | null,
	rendererSelection: readonly SelectionTarget[],
	interactionSelection: readonly SelectionTarget[],
	picked: readonly SelectionTarget[],
	modifiers: InteractionEvent["modifiers"] = {},
): SelectionTarget[] {
	const { inView, outOfView } = replSelectionLayerParts(interactionActive, activeModelDefinitionId, rendererSelection, interactionSelection);
	return [...outOfView, ...replApplySelectionPick(inView, picked, modifiers)];
}

/** @emoji 🪪 Applies archived interaction result to renderer selection when `archiveContext.targets` is set (including `[]`). */
export function replFinalizeSelection(
	rendererSelection: readonly SelectionTarget[],
	result: InteractionSnapshot["lastResponse"],
): readonly SelectionTarget[] {
	const ctx = result?.archiveContext;
	if (!ctx || typeof ctx !== "object" || !Object.hasOwn(ctx, "targets")) return rendererSelection;
	return replInteractionSelectionFromContext(ctx as Record<string, unknown>);
}

/** @emoji 🪩 Memoized `DocumentHistory` for REPL hosts. */
export function useDocumentHistory(): DocumentHistory {
	return useMemo(() => new DocumentHistory(), []);
}

/** @emoji 🪩 Labels + capability mirror for undo/redo chrome (uses `InteractionSnapshot.capabilities`). */
export function getReplHistoryPresentation(
	spec: InteractionSpec,
	snap: InteractionSnapshot,
	history: DocumentHistory,
): { readonly canUndo: boolean; readonly canRedo: boolean; readonly undoLabel: string; readonly redoLabel: string } {
	const active = isInteractionSessionActive(spec, snap.state);
	const u = history.peekUndo()?.label ?? "";
	const r = history.peekRedo()?.label ?? "";
	return {
		canUndo: snap.capabilities.canUndo,
		canRedo: snap.capabilities.canRedo,
		undoLabel: active ? "Interaction input" : u,
		redoLabel: active ? "Interaction input" : r,
	};
}

/** @emoji 🪩 Subscribes to runtime revisions and derives REPL undo/redo labels. */
export function useReplHistoryState(rt: InteractionRuntime, spec: InteractionSpec, history: DocumentHistory) {
	const snap = useInteractionSnapshot(rt);
	return useMemo(() => getReplHistoryPresentation(spec, snap, history), [spec, snap, history]);
}

/** @emoji 🎛️ Optional controlled chrome state for {@link InteractionRepl}. */
export interface InteractionReplHostValues {
	readonly cmdLine?: string;
	readonly activeSuggestionIndex?: number;
	readonly filterKindToggles?: Record<SpatialPickTargetKind, boolean>;
	readonly selectionKindToggles?: Record<SpatialPickTargetKind, boolean>;
	readonly activeModelDefinitionId?: string | null;
	readonly selectionMethod?: SpatialSelectionMethod;
	readonly modelDefinitionRevision?: number;
	readonly dragSelection?: SpatialDragSelectionState | null;
	readonly selectionMenu?: SpatialSelectionRequest | null;
	readonly hoveredPickKey?: string | null;
	readonly rendererSelection?: readonly SelectionTarget[];
	readonly interactionSelection?: readonly SelectionTarget[];
	readonly interactionMenuOpen?: boolean;
	readonly lastFinalizedInteractionId?: string;
}

/** @emoji 📡 Optional `on*` host callbacks for {@link InteractionRepl}. */
export interface InteractionReplHostCallbacks {
	readonly onCmdLineChange?: (value: string) => void;
	readonly onActiveSuggestionIndexChange?: (index: number) => void;
	readonly onFilterKindTogglesChange?: (value: Record<SpatialPickTargetKind, boolean>) => void;
	readonly onSelectionKindTogglesChange?: (value: Record<SpatialPickTargetKind, boolean>) => void;
	readonly onActiveModelDefinitionIdChange?: (value: string) => void;
	readonly onSelectionMethodChange?: (value: SpatialSelectionMethod) => void;
	readonly onModelDefinitionRevisionChange?: (revision: number) => void;
	readonly onDragSelectionChange?: (value: SpatialDragSelectionState | null) => void;
	readonly onSelectionMenuChange?: (value: SpatialSelectionRequest | null) => void;
	readonly onHoveredPickKeyChange?: (key: string | null) => void;
	readonly onRendererSelectionChange?: (targets: readonly SelectionTarget[]) => void;
	readonly onInteractionSelectionChange?: (targets: readonly SelectionTarget[]) => void;
	readonly onInteractionMenuOpenChange?: (open: boolean) => void;
	readonly onLastFinalizedInteractionIdChange?: (id: string) => void;
	readonly onCanvasReady?: InteractionCanvasProps["onCanvasReady"];
	readonly onInteractionEvent?: (event: InteractionEvent) => void;
	readonly onGroundPick?: (point: Vec3, event: InteractionEvent) => void;
	readonly onScenePointerMove?: (point: Vec3, event: InteractionEvent) => void;
	readonly onSelectionRequest?: (request: SpatialSelectionRequest) => void;
	readonly onHoverTarget?: (target: SpatialPickTarget | null) => void;
	readonly onCameraNavigate?: (active: boolean) => void;
	readonly onCommandSubmit?: (line: string) => boolean | void;
	readonly onTransitionRun?: (row: InteractionKeybindRow) => void;
	readonly onCancel?: () => void;
	readonly onUndo?: () => void;
	readonly onRedo?: () => void;
	readonly onSnapshotChange?: (snapshot: InteractionSnapshot) => void;
	readonly onEscape?: () => void;
	readonly onApplyTransformation?: (spec: TransformationSpec) => void;
	/** @emoji 🧲 Geometry used for pick targets (defaults to `geometry`; use builtin geometry when the active model is typology-only). */
	readonly pickGeometry?: SpatialPickGeometry | null;
	readonly onDocumentModelChange?: (model: Model) => void;
}

/** @emoji 📐 Layout and partial canvas/spatial-view overrides for {@link InteractionRepl}. */
export interface InteractionReplLayoutProps {
	readonly rootStyle?: CSSProperties;
	readonly asideStyle?: CSSProperties;
	readonly showAside?: boolean;
	/** @emoji 🙈 Hides model-definition and transformation dropdowns (e.g. play hosts them in `asideExtra`). */
	readonly hideModelDefinitionControls?: boolean;
	readonly frameloop?: InteractionCanvasProps["frameloop"];
	readonly canvas?: Omit<InteractionCanvasProps, "children">;
	/** @emoji 🖼️ Spread after REPL wiring; overrides win (use for theme/slots/face handlers, not session pick state). */
	readonly spatialView?: Omit<
		InteractionSpatialViewProps,
		| "snapshot"
		| "geometry"
		| "committedMeshes"
		| "displayModel"
		| "modelDefinitionRevision"
		| "activeModelDefinitionId"
		| "filterKindToggles"
		| "selectionKindToggles"
		| "hoveredTargetKey"
		| "selectedTargetKey"
		| "selectedTargetKeys"
		| "selectionAccept"
		| "showPickLayer"
		| "pickEnabled"
		| "onInteractionEvent"
		| "onScenePointerMove"
		| "onSelectionRequest"
		| "onCameraNavigate"
		| "onGroundPick"
	>;
}

/** @emoji 🎛️ Default uncontrolled chrome for {@link InteractionRepl}. */
export function defaultInteractionReplChromeState(): Required<
	Pick<
		InteractionReplHostValues,
		| "cmdLine"
		| "activeSuggestionIndex"
		| "filterKindToggles"
		| "selectionKindToggles"
		| "activeModelDefinitionId"
		| "selectionMethod"
		| "modelDefinitionRevision"
		| "dragSelection"
		| "selectionMenu"
		| "hoveredPickKey"
		| "rendererSelection"
		| "interactionSelection"
		| "interactionMenuOpen"
		| "lastFinalizedInteractionId"
	>
> {
	return {
		cmdLine: "",
		activeSuggestionIndex: 0,
		filterKindToggles: defaultSpatialPickKindTogglesForModelDefinition(GEOMETRY_MODEL_DEFINITION_ID),
		selectionKindToggles: defaultSpatialPickKindTogglesForModelDefinition(GEOMETRY_MODEL_DEFINITION_ID),
		activeModelDefinitionId: GEOMETRY_MODEL_DEFINITION_ID,
		selectionMethod: "rectangle",
		modelDefinitionRevision: 0,
		dragSelection: null,
		selectionMenu: null,
		hoveredPickKey: null,
		rendererSelection: [],
		interactionSelection: [],
		interactionMenuOpen: false,
		lastFinalizedInteractionId: "",
	};
}

export interface InteractionReplProps extends InteractionReplHostValues, InteractionReplHostCallbacks, InteractionReplLayoutProps {
	readonly interactionId: string;
	readonly spec: InteractionSpec;
	readonly onInteractionId: (id: string) => void;
	readonly runtime: InteractionRuntime;
	readonly history: DocumentHistory;
	readonly document: ModelDocument;
	readonly geometry: SpatialPickGeometry | null;
	readonly asideExtra?: ReactNode;
	readonly archivedBoxLayouts?: readonly ArchivedBoxLayout[];
	/** @emoji 🔁 When host bumps this positive counter for the same interaction, `cancel()` then `start` without remounting GL. */
	readonly sessionRestartNonce?: number;
	readonly viewTheme?: InteractionSpatialViewTheme;
	readonly viewSlots?: InteractionSpatialViewSlots;
	readonly renderDisplayItem?: SpatialDisplayItemRenderer;
	readonly autoFitMeshes?: boolean;
	readonly autoFitBehavior?: SpatialAutoFitBehavior;
	readonly tessellationTolerance?: number;
}

/** @emoji 🪩 Full spatial REPL: canvas, interaction palette, history controls, last response. */
export function InteractionRepl({
	interactionId,
	spec,
	onInteractionId,
	runtime: rt,
	history,
	document: documentModel,
	geometry,
	asideExtra,
	archivedBoxLayouts = [],
	sessionRestartNonce = 0,
	viewTheme,
	viewSlots,
	renderDisplayItem,
	autoFitMeshes = false,
	autoFitBehavior = "initial",
	tessellationTolerance,
	cmdLine: cmdLineProp,
	activeSuggestionIndex: activeSuggestionIndexProp,
	filterKindToggles: filterKindTogglesProp,
	selectionKindToggles: selectionKindTogglesProp,
	activeModelDefinitionId: activeModelDefinitionIdProp,
	selectionMethod: selectionMethodProp,
	modelDefinitionRevision: modelDefinitionRevisionProp,
	dragSelection: dragSelectionProp,
	selectionMenu: selectionMenuProp,
	hoveredPickKey: hoveredPickKeyProp,
	rendererSelection: rendererSelectionProp,
	interactionSelection: interactionSelectionProp,
	interactionMenuOpen: interactionMenuOpenProp,
	lastFinalizedInteractionId: lastFinalizedInteractionIdProp,
	onCmdLineChange,
	onActiveSuggestionIndexChange,
	onFilterKindTogglesChange,
	onSelectionKindTogglesChange,
	onActiveModelDefinitionIdChange,
	onSelectionMethodChange,
	onModelDefinitionRevisionChange,
	onDragSelectionChange,
	onSelectionMenuChange,
	onHoveredPickKeyChange,
	onRendererSelectionChange,
	onInteractionSelectionChange,
	onInteractionMenuOpenChange,
	onLastFinalizedInteractionIdChange,
	onApplyTransformation,
	pickGeometry: pickGeometryProp,
	onDocumentModelChange,
	onCanvasReady,
	onInteractionEvent: onInteractionEventProp,
	onGroundPick: onGroundPickProp,
	onScenePointerMove: onScenePointerMoveProp,
	onSelectionRequest: onSelectionRequestProp,
	onHoverTarget: onHoverTargetProp,
	onCameraNavigate: onCameraNavigateProp,
	onCommandSubmit,
	onTransitionRun,
	onCancel,
	onUndo,
	onRedo,
	onSnapshotChange,
	onEscape,
	rootStyle,
	asideStyle,
	showAside = true,
	hideModelDefinitionControls = false,
	frameloop = "always",
	canvas: canvasOverrides,
	spatialView: spatialViewOverrides,
}: InteractionReplProps): ReactNode {
	const snapshot = useInteractionSnapshot(rt);
	const tessTolerance = tessellationTolerance ?? (rt.computeMode() === "fast" ? 0.02 : 0.0008);
	const committedMeshes = useDocumentMeshes(rt.kernel(), documentModel.model, tessTolerance);
	const documentArchivedBoxLayouts = useMemo(() => archivedBoxesFromHistory(history), [history, snapshot.revision]);
	const allArchivedBoxLayouts = useMemo(
		() => [...documentArchivedBoxLayouts, ...archivedBoxLayouts],
		[documentArchivedBoxLayouts, archivedBoxLayouts],
	);
	const baseDisplay = useMemo(() => replBaseDisplayForHistory(snapshot), [snapshot]);
	const mergedDisplay = useMemo(
		() => mergeDisplayWithArchivedBoxes(baseDisplay, allArchivedBoxLayouts),
		[baseDisplay, allArchivedBoxLayouts],
	);
	const chromeDefaults = useMemo(() => defaultInteractionReplChromeState(), []);
	const [cmdLine, setCmdLine] = useHostState(cmdLineProp, onCmdLineChange, () => chromeDefaults.cmdLine);
	const [activeIndex, setActiveIndex] = useHostState(activeSuggestionIndexProp, onActiveSuggestionIndexChange, () => chromeDefaults.activeSuggestionIndex);
	const [filterKindToggles, setFilterKindToggles] = useHostState(filterKindTogglesProp, onFilterKindTogglesChange, () => chromeDefaults.filterKindToggles);
	const [selectionKindToggles, setSelectionKindToggles] = useHostState(selectionKindTogglesProp, onSelectionKindTogglesChange, () => chromeDefaults.selectionKindToggles);
	const [activeModelDefinitionId, setActiveModelDefinitionId] = useHostState(
		activeModelDefinitionIdProp,
		onActiveModelDefinitionIdChange,
		() => chromeDefaults.activeModelDefinitionId,
	);
	const mdIdForView = activeModelDefinitionId ?? GEOMETRY_MODEL_DEFINITION_ID;
	const committedMeshesForView = useMemo(
		() => (modelDefinitionUsesGeometryPicking(mdIdForView) ? committedMeshes : []),
		[committedMeshes, mdIdForView],
	);
	const [selectionMethod, setSelectionMethod] = useHostState(selectionMethodProp, onSelectionMethodChange, () => chromeDefaults.selectionMethod);
	const [modelDefinitionRevision, setModelDefinitionRevision] = useHostState(
		modelDefinitionRevisionProp,
		onModelDefinitionRevisionChange,
		() => chromeDefaults.modelDefinitionRevision,
	);
	const modelDefinitions = useMemo(() => listModelDefinitionManifests(), []);
	const transformsFrom = useMemo(
		() => listTransformationsIntoModelDefinition(activeModelDefinitionId ?? GEOMETRY_MODEL_DEFINITION_ID),
		[activeModelDefinitionId],
	);
	const transformsTo = useMemo(
		() => listTransformationsFromModelDefinition(activeModelDefinitionId ?? GEOMETRY_MODEL_DEFINITION_ID),
		[activeModelDefinitionId],
	);
	const modelDefinitionScope = useMemo(
		() => resolveModelDefinitionScope(activeModelDefinitionId ?? GEOMETRY_MODEL_DEFINITION_ID),
		[activeModelDefinitionId],
	);
	const scopedInteractions = useMemo(
		() => listSpatialInteractionsForModelDefinition(activeModelDefinitionId ?? GEOMETRY_MODEL_DEFINITION_ID),
		[activeModelDefinitionId, modelDefinitionRevision],
	);
	const kernel = rt.kernel();
	const [dragSelection, setDragSelection] = useHostState(dragSelectionProp, onDragSelectionChange, () => chromeDefaults.dragSelection);
	const [selectionMenu, setSelectionMenu] = useHostState(selectionMenuProp, onSelectionMenuChange, () => chromeDefaults.selectionMenu);
	const [hoveredPickKey, setHoveredPickKey] = useHostState(hoveredPickKeyProp, onHoveredPickKeyChange, () => chromeDefaults.hoveredPickKey);
	const [rendererSelection, setRendererSelection] = useHostState(
		rendererSelectionProp,
		onRendererSelectionChange,
		() => [...chromeDefaults.rendererSelection],
	);
	const [interactionSelection, setInteractionSelection] = useHostState(
		interactionSelectionProp,
		onInteractionSelectionChange,
		() => [...chromeDefaults.interactionSelection],
	);
	const [interactionMenuOpen, setInteractionMenuOpen] = useHostState(interactionMenuOpenProp, onInteractionMenuOpenChange, () => chromeDefaults.interactionMenuOpen);
	const [lastFinalizedInteractionId, setLastFinalizedInteractionId] = useHostState(
		lastFinalizedInteractionIdProp,
		onLastFinalizedInteractionIdChange,
		() => chromeDefaults.lastFinalizedInteractionId,
	);
	const [canvasBinding, setCanvasBinding] = useState<{ readonly camera: THREE.Camera; readonly domElement: HTMLCanvasElement } | null>(null);
	const handleCanvasReady = useCallback(
		(binding: { readonly camera: THREE.Camera; readonly domElement: HTMLCanvasElement }) => {
			setCanvasBinding(binding);
			onCanvasReady?.(binding);
		},
		[onCanvasReady],
	);
	useEffect(() => {
		onSnapshotChange?.(snapshot);
	}, [snapshot, onSnapshotChange]);
	const cmdRef = useRef<HTMLInputElement>(null);
	const setCmdLineRef = useRef(setCmdLine);
	const rendererSelectionRef = useRef(rendererSelection);
	const suppressAutoStartOnceRef = useRef(false);
	const lastViewsRefreshRef = useRef<{ readonly model: Model | null; readonly revision: number; readonly activeModelDefinitionId: string | null }>({
		model: null,
		revision: -1,
		activeModelDefinitionId: null,
	});
	const dragSelectionRef = useRef<SpatialDragSelectionState | null>(null);
	const dragCleanupRef = useRef<(() => void) | null>(null);
	const cameraNavigatingRef = useRef(false);
	const interactionActive = isInteractionSessionActive(spec, snapshot.state);
	const displayedSelectionTargets = useMemo(
		() => replDisplayedSelectionTargets(interactionActive, activeModelDefinitionId, rendererSelection, interactionSelection),
		[interactionActive, activeModelDefinitionId, rendererSelection, interactionSelection],
	);
	const viewFilterKindToggles = useMemo((): SpatialPickKindToggles => {
		const allowed = new Set(modelDefinitionPickTargetKinds(activeModelDefinitionId));
		const merged: SpatialPickKindToggles = {};
		for (const kind of SPATIAL_PICK_TARGET_KINDS) {
			merged[kind] = allowed.has(kind) && filterKindToggles[kind] !== false;
		}
		return merged;
	}, [filterKindToggles, activeModelDefinitionId]);
	const effectiveSelectionKindToggles = useMemo(
		() => intersectSpatialPickKindToggles(viewFilterKindToggles, selectionKindToggles),
		[selectionKindToggles, viewFilterKindToggles],
	);
	const selectedPickKeys = useMemo(() => new Set(displayedSelectionTargets.map(spatialSelectionTargetKey)), [displayedSelectionTargets]);
	const selectedPickKey = displayedSelectionTargets[0] ? spatialSelectionTargetKey(displayedSelectionTargets[0]) : null;
	const geometryPreviewTransform = useMemo(() => geometryPreviewTransformFromDisplay(mergedDisplay), [mergedDisplay]);
	const pickGeometry = pickGeometryProp ?? geometry;
	const pickGeometryRevision =
		pickGeometry && typeof pickGeometry === "object" && "revision" in pickGeometry
			? Number((pickGeometry as { revision?: unknown }).revision)
			: 0;
	const pickTargets = useMemo(
		() => createSpatialPickTargets(pickGeometry, activeModelDefinitionId),
		[pickGeometry, pickGeometryRevision, modelDefinitionRevision, activeModelDefinitionId],
	);
	const scopedPickTargets = useMemo(() => filterSpatialPickTargetsForActiveView(pickTargets, activeModelDefinitionId), [pickTargets, activeModelDefinitionId]);
	useEffect(() => {
		setCmdLineRef.current = setCmdLine;
	}, [setCmdLine]);

	useEffect(() => {
		rendererSelectionRef.current = rendererSelection;
	}, [rendererSelection]);

	const dismissReplChrome = useCallback(() => {
		dragCleanupRef.current?.();
		dragCleanupRef.current = null;
		dragSelectionRef.current = null;
		setDragSelection(null);
		setCmdLine("");
		setSelectionMenu(null);
		setHoveredPickKey(null);
		setInteractionMenuOpen(false);
	}, []);

	const cancelActiveInteraction = useCallback(() => {
		const aborted = abortActiveInteractionSession(rt);
		if (!aborted && !interactionId) return false;
		if (!aborted) rt.cancel();
		suppressAutoStartOnceRef.current = true;
		setInteractionSelection([]);
		dismissReplChrome();
		if (interactionId) onInteractionId("");
		onCancel?.();
		return true;
	}, [rt, interactionId, onInteractionId, dismissReplChrome, onCancel, setInteractionSelection]);

	useEffect(() => {
		if (!interactionId || !snapshot.lastResponse?.ok) return;
		setLastFinalizedInteractionId(interactionId);
		setRendererSelection((prev) => [...replFinalizeSelection(prev, snapshot.lastResponse)]);
		setInteractionSelection((prev) => (prev.length === 0 ? prev : []));
	}, [interactionId, snapshot.lastResponse, setInteractionSelection, setLastFinalizedInteractionId, setRendererSelection]);

	const handleEscapeKey = useCallback(() => {
		if (selectionMenu !== null) {
			setSelectionMenu(null);
			setHoveredPickKey(null);
			onEscape?.();
			return;
		}
		switch (replEscapeAction({ hasInteraction: Boolean(interactionId), interactionActive, cmdLine, hasSelectionMenu: selectionMenu !== null })) {
			case "abort":
				cancelActiveInteraction();
				onEscape?.();
				return;
			case "dismiss":
				dismissReplChrome();
				onEscape?.();
				return;
			default:
				return;
		}
	}, [interactionId, interactionActive, cmdLine, selectionMenu, dismissReplChrome, cancelActiveInteraction, onEscape, setSelectionMenu, setHoveredPickKey]);

	const startRuntime = useCallback(async () => {
		const accept = rt.listActiveSelectionAccept() as readonly ModelEntityKind[];
		const accepted = replSelectionAccepted(accept, rendererSelectionRef.current);
		setInteractionSelection(accepted);
		await rt.send(replStartEvent(accepted));
	}, [rt, setInteractionSelection]);

	useEffect(() => {
		if (!interactionId) return;
		if (suppressAutoStartOnceRef.current) {
			suppressAutoStartOnceRef.current = false;
			return;
		}
		void startRuntime();
	}, [interactionId, startRuntime]);

	useEffect(() => {
		if (sessionRestartNonce <= 0) return;
		rt.cancel();
		void startRuntime();
	}, [sessionRestartNonce, rt, startRuntime]);

	const repeatCurrentInteraction = useCallback(() => {
		rt.cancel();
		void startRuntime();
	}, [rt, startRuntime]);

	const modelRevision = documentModel.model.revision;
	const hostPickingEnabled = replHostGeometryPickingEnabled(interactionId, spec, snapshot.state);

	useEffect(() => {
		setSelectionMenu(null);
		setHoveredPickKey(null);
	}, [geometry, snapshot.state, modelDefinitionRevision]);

	useEffect(() => {
		const mdId = activeModelDefinitionId ?? GEOMETRY_MODEL_DEFINITION_ID;
		const defaults = defaultSpatialPickKindTogglesForModelDefinition(mdId);
		setFilterKindToggles(defaults);
		setSelectionKindToggles(defaults);
		const allowed = listSpatialInteractionsForModelDefinition(mdId);
		if (interactionId && !allowed.some((row) => row.id === interactionId)) onInteractionId("");
		setLastFinalizedInteractionId("");
		setRendererSelection((prev) => (prev.length === 0 ? prev : []));
		setInteractionSelection((prev) => (prev.length === 0 ? prev : []));
	}, [activeModelDefinitionId, modelDefinitionRevision, interactionId, onInteractionId, setFilterKindToggles, setSelectionKindToggles, setRendererSelection, setInteractionSelection, setLastFinalizedInteractionId]);

	useEffect(() => {
		setRendererSelection((prev) => (prev.length === 0 ? prev : []));
		setInteractionSelection((prev) => (prev.length === 0 ? prev : []));
	}, [geometry, modelDefinitionRevision, setRendererSelection, setInteractionSelection]);

	useEffect(() => {
		setCmdLine("");
		setActiveIndex(0);
		setSelectionMenu(null);
		setHoveredPickKey(null);
		setInteractionMenuOpen(false);
		setInteractionSelection((prev) => (prev.length === 0 ? prev : []));
	}, [interactionId, rt, setInteractionSelection]);

	const confirmInteractionSelection = useCallback(() => {
		const snap = rt.getSnapshot();
		if (!interactionCanConfirmSelection(spec, snap.state, snap.context, scenePreview())) return false;
		void rt.send({ kind: "confirm", modifiers: {} });
		return true;
	}, [rt, spec]);

	useEffect(() => {
		if (!interactionActive) {
			setInteractionSelection((prev) => (prev.length === 0 ? prev : []));
			return;
		}
		const machineTargets = replInteractionSelectionFromContext(snapshot.context);
		setInteractionSelection((prev) => (replSelectionTargetsEqual(prev, machineTargets) ? prev : [...machineTargets]));
	}, [interactionActive, snapshot.revision, snapshot.context, setInteractionSelection]);

	const runtimeSelectionAccept = useMemo(() => rt.listActiveSelectionAccept(), [rt, snapshot.state]);
	const defaultSelectionAccept = useMemo(
		() => modelDefinitionSelectionEntityKinds(activeModelDefinitionId ?? GEOMETRY_MODEL_DEFINITION_ID),
		[activeModelDefinitionId],
	);
	const activeSelectionAccept = useMemo((): readonly ModelEntityKind[] => {
		if (runtimeSelectionAccept.length > 0) {
			const allowed = new Set(defaultSelectionAccept);
			return runtimeSelectionAccept.filter((kind) => allowed.has(kind));
		}
		if (interactionActive) return [];
		return defaultSelectionAccept;
	}, [runtimeSelectionAccept, interactionActive, defaultSelectionAccept]);
	const activePickKinds = useMemo(() => [...spatialPickKindsForActiveView(activeModelDefinitionId)], [activeModelDefinitionId]);
	const viewObjectCount = useMemo(() => {
		if (isGeometryModelDefinition(activeModelDefinitionId)) return 0;
		return countViewObjectsForModelDefinition(documentModel.model, mdIdForView);
	}, [activeModelDefinitionId, documentModel.model, mdIdForView, modelDefinitionRevision]);

	const commitSelection = useCallback(
		(selection: readonly SelectionTarget[]) => {
			setSelectionMenu(null);
			setHoveredPickKey(null);
			if (interactionActive) setInteractionSelection([...selection]);
			else setRendererSelection([...selection]);
		},
		[interactionActive, setInteractionSelection, setRendererSelection],
	);

	const applySelectionPrune = useCallback(
		(map: (selection: readonly SelectionTarget[]) => readonly SelectionTarget[]) => {
			setRendererSelection((prev) => [...map(prev)]);
			setInteractionSelection((prev) => [...map(prev)]);
		},
		[setRendererSelection, setInteractionSelection],
	);

	const dispatchSelectionTargets = useCallback(
		(targets: readonly SpatialPickTarget[], modifiers: InteractionEvent["modifiers"] = {}, point?: Vec3) => {
			const picked = uniqueSelectionTargets(targets.map(spatialSelectionTarget));
			const nextSelection = replMergeSelectionPickInView(
				interactionActive,
				activeModelDefinitionId,
				rendererSelection,
				interactionSelection,
				picked,
				modifiers,
			);
			commitSelection(nextSelection);
			if (interactionActive && picked.length > 0) void rt.send({ ...replSelectionEvent(picked, point), modifiers });
		},
		[commitSelection, interactionActive, interactionSelection, activeModelDefinitionId, rt, rendererSelection],
	);

	const onSelectionRequest = useCallback(
		(request: SpatialSelectionRequest) => {
			onSelectionRequestProp?.(request);
			if (request.targets.length === 1) {
				dispatchSelectionTargets([request.targets[0]!], request.modifiers, request.point);
				return;
			}
			setSelectionMenu(request);
			setHoveredPickKey(request.targets[0] ? spatialPickTargetKey(request.targets[0]) : null);
		},
		[dispatchSelectionTargets, onSelectionRequestProp, setSelectionMenu, setHoveredPickKey],
	);

	const onHoverTarget = useCallback(
		(target: SpatialPickTarget | null) => {
			const key = target ? spatialPickTargetKey(target) : null;
			setHoveredPickKey((prev) => (prev === key ? prev : key));
			onHoverTargetProp?.(target);
		},
		[onHoverTargetProp, setHoveredPickKey],
	);

	const onCameraNavigate = useCallback(
		(active: boolean) => {
			cameraNavigatingRef.current = active;
			if (active) onHoverTarget(null);
			onCameraNavigateProp?.(active);
		},
		[onHoverTarget, onCameraNavigateProp],
	);

	useEffect(() => {
		const canvas = canvasBinding?.domElement;
		const camera = canvasBinding?.camera;
		if (!canvas || !camera || !hostPickingEnabled) return;
		let lastHoverAt = 0;
		const onMove = (event: PointerEvent) => {
			if (cameraNavigatingRef.current || event.buttons !== 0) {
				onHoverTarget(null);
				return;
			}
			const now = performance.now();
			if (now - lastHoverAt < 32) return;
			lastHoverAt = now;
			const rect = canvas.getBoundingClientRect();
			const hits = spatialPickTargetsFromClientPoint(
				{ x: event.clientX, y: event.clientY },
				camera,
				rect,
				scopedPickTargets,
				[],
				effectiveSelectionKindToggles,
			);
			onHoverTarget(hits[0] ?? null);
		};
		const onLeave = () => onHoverTarget(null);
		canvas.addEventListener("pointermove", onMove, { passive: true });
		canvas.addEventListener("pointerleave", onLeave, { passive: true });
		return () => {
			canvas.removeEventListener("pointermove", onMove);
			canvas.removeEventListener("pointerleave", onLeave);
		};
	}, [canvasBinding, hostPickingEnabled, scopedPickTargets, effectiveSelectionKindToggles, onHoverTarget]);

	const pointerMoveActive = useMemo(() => {
		const si = snapshot.spatialInteraction;
		return (
			si.spatialGroundPick &&
			(si.groundPointerMoveStates.includes(snapshot.state) ||
				si.heightDragStates.includes(snapshot.state) ||
				si.verticalRodStates.includes(snapshot.state))
		);
	}, [snapshot.state, snapshot.spatialInteraction]);

	const onSpatialInteractionEvent = useCallback(
		(ev: InteractionEvent) => {
			onInteractionEventProp?.(ev);
			if (ev.kind === "pointer.down") {
				const st = rt.getSnapshot().state;
				const hi = rt.getSnapshot().spatialInteraction.heightConfirmState;
				const snapEv = (ev as { snap?: { kind: string; id: string } }).snap;
				if (hi && st === hi && !snapEv) {
					void rt.send({ kind: "confirm", modifiers: (ev as { modifiers?: Record<string, unknown> }).modifiers ?? {} });
					return;
				}
				if (
					snapEv &&
					activeSelectionAccept.length > 0 &&
					activeSelectionAccept.includes(snapEv.kind as ModelEntityKind) &&
					effectiveSelectionKindToggles[snapEv.kind as SpatialPickTargetKind] !== false
				) {
					const snapTarget: SpatialPickTarget = {
						kind: snapEv.kind as SpatialPickTargetKind,
						id: snapEv.id,
						point: (ev as { point?: Vec3 }).point ?? [0, 0, 0],
					};
					const selection = spatialSelectionTarget(snapTarget);
					const modifiers = (ev as { modifiers?: InteractionEvent["modifiers"] }).modifiers ?? {};
					commitSelection(
						replMergeSelectionPickInView(
							interactionActive,
							activeModelDefinitionId,
							rendererSelection,
							interactionSelection,
							[selection],
							modifiers,
						),
					);
					if (interactionActive) void rt.send({ ...replSelectionEvent([selection], (ev as { point?: Vec3 }).point), modifiers });
					return;
				}
			}
			if (ev.kind === "pointer.move" && !pointerMoveActive) return;
			if (ev.kind === "pointer.down" || ev.kind === "pointer.move" || ev.kind === "contextmenu") void rt.send(ev);
		},
		[
			rt,
			activeSelectionAccept,
			commitSelection,
			interactionActive,
			interactionSelection,
			rendererSelection,
			pointerMoveActive,
			activeModelDefinitionId,
			effectiveSelectionKindToggles,
			onInteractionEventProp,
		],
	);

	useEffect(() => {
		const canvas = canvasBinding?.domElement;
		const camera = canvasBinding?.camera;
		if (!canvas || !camera || !hostPickingEnabled || activeSelectionAccept.length === 0) return;
		const clearDragSelection = () => {
			dragCleanupRef.current = null;
			dragSelectionRef.current = null;
			setDragSelection(null);
		};
		const beginDragSelection = (event: PointerEvent) => {
			if (event.button !== 0) return;
			dragCleanupRef.current?.();
			const rect = canvas.getBoundingClientRect();
			const startClient = { x: event.clientX, y: event.clientY };
			const initial: SpatialDragSelectionState = {
				method: selectionMethod,
				coverage: "full",
				startClient,
				currentClient: startClient,
				path: [startClient],
				modifiers: pointerModifiersFromNativeEvent(event),
			};
			dragSelectionRef.current = initial;
			const moveSelection = (moveEvent: PointerEvent) => {
				const current = dragSelectionRef.current;
				if (!current) return;
				const nextClient = { x: moveEvent.clientX, y: moveEvent.clientY };
				const nextPath =
					current.method === "lasso" && dragDistance(current.path[current.path.length - 1]!, nextClient) >= 2
						? [...current.path, nextClient]
						: current.method === "lasso"
							? current.path
							: [current.startClient, nextClient];
				const nextState: SpatialDragSelectionState = {
					...current,
					currentClient: nextClient,
					path: nextPath,
					coverage: spatialSelectionCoverageFromPath(nextPath),
					modifiers: pointerModifiersFromNativeEvent(moveEvent),
				};
				dragSelectionRef.current = nextState;
				if (dragDistance(nextState.startClient, nextClient) >= 4) setDragSelection(nextState);
			};
			const finishSelection = (upEvent: PointerEvent) => {
				window.removeEventListener("pointermove", moveSelection, true);
				window.removeEventListener("pointerup", finishSelection, true);
				const current = dragSelectionRef.current;
				clearDragSelection();
				if (!current) return;
				const finalState: SpatialDragSelectionState = {
					...current,
					currentClient: { x: upEvent.clientX, y: upEvent.clientY },
					path:
						current.method === "lasso"
							? [...current.path, { x: upEvent.clientX, y: upEvent.clientY }]
							: [current.startClient, { x: upEvent.clientX, y: upEvent.clientY }],
					modifiers: pointerModifiersFromNativeEvent(upEvent),
				};
				const distance = dragDistance(finalState.startClient, finalState.currentClient);
				if (distance < 4) {
					const candidates = spatialPickTargetsFromClientPoint(
						finalState.currentClient,
						camera,
						rect,
						scopedPickTargets,
						activeSelectionAccept,
						effectiveSelectionKindToggles,
					);
					if (candidates.length === 0) return;
					onSelectionRequest({
						targets: candidates,
						point: candidates[0]!.point,
						client: finalState.currentClient,
						modifiers: finalState.modifiers,
					});
					return;
				}
				const targets = spatialPickTargetsFromScreenSelection(
					{ ...finalState, coverage: spatialSelectionCoverageFromPath(finalState.path) },
					scopedPickTargets,
					camera,
					canvas.getBoundingClientRect(),
					activeSelectionAccept,
					effectiveSelectionKindToggles,
					geometryPreviewTransform,
				);
				if (targets.length === 0) {
					if (
						spatialSelectionModeFromModifiers(
							finalState.modifiers as { readonly alt?: boolean; readonly ctrl?: boolean; readonly meta?: boolean; readonly shift?: boolean },
						) === "default"
					) {
						commitSelection(
							replMergeSelectionPickInView(interactionActive, activeModelDefinitionId, rendererSelection, interactionSelection, [], finalState.modifiers),
						);
					}
					return;
				}
				dispatchSelectionTargets(targets, finalState.modifiers);
			};
			dragCleanupRef.current = () => {
				window.removeEventListener("pointermove", moveSelection, true);
				window.removeEventListener("pointerup", finishSelection, true);
				clearDragSelection();
			};
			window.addEventListener("pointermove", moveSelection, true);
			window.addEventListener("pointerup", finishSelection, true);
		};
		canvas.addEventListener("pointerdown", beginDragSelection, true);
		return () => {
			dragCleanupRef.current?.();
			canvas.removeEventListener("pointerdown", beginDragSelection, true);
		};
	}, [
		activeSelectionAccept,
		canvasBinding,
		commitSelection,
		dispatchSelectionTargets,
		interactionActive,
		interactionSelection,
		onSelectionRequest,
		activeModelDefinitionId,
		rendererSelection,
		selectionMethod,
		geometryPreviewTransform,
		effectiveSelectionKindToggles,
		hostPickingEnabled,
	]);

	const dispatchTransition = useCallback(
		(row: InteractionKeybindRow) => {
			onTransitionRun?.(row);
			const ev = replBuildDispatchEvent(row, { interactionId: spec.id, model: documentModel.model });
			if (ev) void rt.send(ev);
		},
		[rt, spec.id, documentModel.model, onTransitionRun],
	);

	const transitionRows = useMemo(() => listKeyedInteractionTransitions(spec, snapshot.state), [spec, snapshot.state]);

	const allSuggestions = useMemo((): ReplSuggestion[] => {
		const out: ReplSuggestion[] = [];
		for (const p of scopedInteractions) {
			out.push({
				kind: "interaction",
				key: p.key,
				label: p.label,
				detail: p.id,
				interactionId: p.id,
				onRun: () => onInteractionId(p.id),
			});
		}
		for (const row of transitionRows) {
			out.push({
				kind: "transition",
				key: row.key,
				label: row.label,
				detail: row.eventKind,
				transition: row,
				onRun: () => dispatchTransition(row),
			});
		}
		for (const defn of modelDefinitionScope.selectionOperations) {
			out.push({
				kind: "selection",
				key: defn.key,
				label: defn.label,
				detail: defn.id,
				onRun: () => {
					void rt.query(`CALL ${defn.id}({}) YIELD data.targets AS targets`);
				},
			});
		}
		for (const actionId of modelDefinitionScope.actions) {
			if (actionId.startsWith("selection.")) continue;
			const tail = actionId.includes(".") ? actionId.slice(actionId.lastIndexOf(".") + 1) : actionId;
			out.push({
				kind: "action",
				key: tail,
				label: actionId,
				detail: "action",
				onRun: () => {
					void rt.query(`CALL ${actionId}({})`);
				},
			});
		}
		return out;
	}, [scopedInteractions, transitionRows, modelDefinitionScope, onInteractionId, dispatchTransition, rt]);

	const filtered = useMemo(() => replPaletteRows(cmdLine, allSuggestions), [cmdLine, allSuggestions]);
	const interactionMatches = useMemo(() => replInteractionSuggestions(cmdLine, allSuggestions), [cmdLine, allSuggestions]);
	const completionSuffix = useMemo(
		() => replActiveCompletionSuffix(cmdLine, filtered, activeIndex),
		[cmdLine, filtered, activeIndex],
	);

	useEffect(() => {
		setActiveIndex((i) => (filtered.length ? Math.min(i, filtered.length - 1) : 0));
	}, [filtered.length, cmdLine]);

	const runSuggestion = useCallback((s: ReplSuggestion) => {
		s.onRun();
		setCmdLine("");
		setActiveIndex(0);
		setInteractionMenuOpen(false);
	}, []);

	const runInteractionIdFromSpace = useCallback(
		(id: string | null): boolean => {
			if (!id) return false;
			onInteractionId(id);
			setCmdLine("");
			setActiveIndex(0);
			setInteractionMenuOpen(false);
			return true;
		},
		[onInteractionId],
	);

	const trySubmitLine = useCallback((): boolean => {
		const raw = cmdLine.trim();
		if (!raw) return false;
		if (onCommandSubmit?.(raw)) {
			setCmdLine("");
			return true;
		}
		const valEv = replTryParseValueInteraction(raw, spec, rt.getSnapshot().state);
		if (valEv) {
			void rt.send(valEv);
			setCmdLine("");
			return true;
		}
		const interactionHit = resolveScopedSpatialInteractionKey(raw, activeModelDefinitionId ?? GEOMETRY_MODEL_DEFINITION_ID);
		if (interactionHit) {
			onInteractionId(interactionHit.id);
			setCmdLine("");
			return true;
		}
		const rows = listKeyedInteractionTransitions(spec, rt.getSnapshot().state);
		for (const row of rows) {
			if (row.eventKind === "set.height" || row.eventKind === "set.distance" || row.eventKind === "set.footprint") continue;
			if (row.key === raw || row.key.toLowerCase() === raw.toLowerCase() || row.eventKind.toLowerCase() === raw.toLowerCase()) {
				dispatchTransition(row);
				setCmdLine("");
				return true;
			}
		}
		return false;
	}, [cmdLine, spec, rt, dispatchTransition, onInteractionId, onCommandSubmit, setCmdLine, activeModelDefinitionId]);

	const runTransitionRow = useCallback(
		(row: InteractionKeybindRow) => {
			if (row.eventKind.startsWith("set.")) {
				setCmdLine(row.key);
				window.setTimeout(() => cmdRef.current?.focus(), 0);
				return;
			}
			dispatchTransition(row);
		},
		[dispatchTransition],
	);

	const onInputKeyDown = useCallback(
		(e: KeyboardEvent<HTMLInputElement>) => {
			if (e.key === "Escape") {
				e.preventDefault();
				handleEscapeKey();
				return;
			}
			if (e.key === " " && !e.ctrlKey && !e.metaKey && !e.altKey) {
				e.preventDefault();
				const interactionIdOnSpace = replInteractionIdOnSpace(cmdLine, filtered, allSuggestions, lastFinalizedInteractionId);
				if (runInteractionIdFromSpace(interactionIdOnSpace)) return;
				setInteractionMenuOpen(false);
				return;
			}
			if (e.key === "ArrowDown" && filtered.length) {
				e.preventDefault();
				setInteractionMenuOpen(false);
				setActiveIndex((i) => (i + 1) % filtered.length);
				return;
			}
			if (e.key === "ArrowUp" && filtered.length) {
				e.preventDefault();
				setInteractionMenuOpen(false);
				setActiveIndex((i) => (i - 1 + filtered.length) % filtered.length);
				return;
			}
			if (e.key === "Tab" && filtered.length) {
				e.preventDefault();
				const suffix = replActiveCompletionSuffix(cmdLine, filtered, activeIndex);
				if (suffix) {
					setCmdLine(replCommandTextWithoutSpaces(cmdLine + suffix));
					return;
				}
				runSuggestion(filtered[activeIndex] ?? filtered[0]!);
				return;
			}
			if (e.key === "Enter") {
				e.preventDefault();
				setInteractionMenuOpen(false);
				if (!cmdLine.trim() && confirmInteractionSelection()) return;
				if (trySubmitLine()) return;
				if (filtered.length) runSuggestion(filtered[activeIndex]!);
				return;
			}
		},
		[cmdLine, allSuggestions, filtered, activeIndex, runSuggestion, trySubmitLine, handleEscapeKey, lastFinalizedInteractionId, runInteractionIdFromSpace, confirmInteractionSelection],
	);

	useEffect(() => {
		const onWinCapture = (e: globalThis.KeyboardEvent) => {
			if (e.defaultPrevented || e.isComposing) return;
			const t = e.target;
			const one = e.key.length === 1 ? e.key : "";
			if (replIsQueryTypingTarget(t)) return;
			if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "z") {
				e.preventDefault();
				e.stopPropagation();
				if (e.shiftKey) {
					rt.redo();
					onRedo?.();
				} else {
					rt.undo();
					onUndo?.();
				}
				return;
			}
			if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "y") {
				e.preventDefault();
				e.stopPropagation();
				rt.redo();
				onRedo?.();
				return;
			}
			if (e.key === " " && !e.ctrlKey && !e.metaKey && !e.altKey) {
				e.preventDefault();
				e.stopPropagation();
				if (!cmdLine.trim() && confirmInteractionSelection()) return;
				const matches = replPaletteRows(cmdLine, allSuggestions);
				const interactionIdOnSpace = replInteractionIdOnSpace(cmdLine, matches, allSuggestions, lastFinalizedInteractionId);
				if (runInteractionIdFromSpace(interactionIdOnSpace)) return;
				else if (replShouldRepeatInteractionOnSpace(e, { interactionActive, cmdTarget: cmdRef.current })) repeatCurrentInteraction();
				return;
			}
			if (t !== cmdRef.current && e.key === "Backspace") {
				e.preventDefault();
				e.stopPropagation();
				cmdRef.current?.focus();
				setCmdLineRef.current((prev) => prev.slice(0, -1));
				return;
			}
			if (t !== cmdRef.current && e.key === "Escape") {
				e.preventDefault();
				e.stopPropagation();
				cmdRef.current?.focus();
				handleEscapeKey();
				return;
			}
			if (t !== cmdRef.current && e.key === "Enter") {
				e.preventDefault();
				e.stopPropagation();
				if (!cmdLine.trim() && confirmInteractionSelection()) return;
				cmdRef.current?.focus();
				if (cmdLine.trim()) void trySubmitLine();
				return;
			}
			if (!one || e.ctrlKey || e.metaKey || e.altKey) return;
			if (t === cmdRef.current) return;
			e.preventDefault();
			e.stopPropagation();
			cmdRef.current?.focus();
			setCmdLineRef.current((prev) => replCommandTextWithoutSpaces(`${prev}${one}`));
		};
		window.addEventListener("keydown", onWinCapture, true);
		return () => window.removeEventListener("keydown", onWinCapture, true);
	}, [rt, cmdLine, allSuggestions, trySubmitLine, handleEscapeKey, interactionActive, repeatCurrentInteraction, lastFinalizedInteractionId, runInteractionIdFromSpace, confirmInteractionSelection, onUndo, onRedo]);

	const onScenePointerMove = useCallback(
		(p: Vec3) => {
			const event = createSpatialPickEvent("pointer.move", p, null);
			void rt.send(event);
			onScenePointerMoveProp?.(p, event);
		},
		[rt, onScenePointerMoveProp],
	);

	const pickPlaneOn = snapshot.spatialInteraction.spatialGroundPick
		? !snapshot.spatialInteraction.pickDisabledStates.includes(snapshot.state)
		: false;

	const lr = snapshot.lastResponse;
	const dragOverlayRect = canvasBinding?.domElement.getBoundingClientRect() ?? null;
	const dragOverlayPoints =
		dragSelection && dragOverlayRect
			? dragSelection.path.map((point) => ({ x: point.x - dragOverlayRect.left, y: point.y - dragOverlayRect.top }))
			: [];

	return (
		<div
			style={{
				display: "flex",
				height: "100vh",
				fontFamily: "system-ui",
				color: "#e8e8f0",
				background: "#080810",
				...rootStyle,
			}}
		>
			<div style={{ flex: 1, minWidth: 0, position: "relative" }}>
				<InteractionCanvas {...canvasOverrides} frameloop={frameloop} onCanvasReady={handleCanvasReady}>
					<InteractionSpatialView
						previewKernel={rt.previewKernel()}
						snapshot={snapshot}
						onInteractionEvent={onSpatialInteractionEvent}
						onGroundPick={onGroundPickProp}
						onScenePointerMove={pointerMoveActive ? onScenePointerMove : undefined}
						pickEnabled={pickPlaneOn}
						geometry={geometry}
						pickGeometry={pickGeometry}
						committedMeshes={committedMeshesForView}
						activeModelDefinitionId={activeModelDefinitionId}
						modelDefinitionRevision={modelDefinitionRevision}
						displayModel={mergedDisplay}
						renderDisplayItem={renderDisplayItem}
						selectionAccept={hostPickingEnabled ? activeSelectionAccept : []}
						filterKindToggles={filterKindToggles}
						selectionKindToggles={effectiveSelectionKindToggles}
						hoveredTargetKey={hoveredPickKey}
						selectedTargetKey={selectedPickKey}
						selectedTargetKeys={selectedPickKeys}
						hostSelectionEnabled={hostPickingEnabled}
						showPickLayer={hostPickingEnabled}
						onSelectionRequest={onSelectionRequest}
						onCameraNavigate={onCameraNavigate}
						autoFitMeshes={autoFitMeshes}
						autoFitBehavior={autoFitBehavior}
						theme={viewTheme}
						slots={viewSlots}
						{...spatialViewOverrides}
					/>
				</InteractionCanvas>
				{dragSelection && dragOverlayRect ? (
					<svg
						width="100%"
						height="100%"
						style={{
							position: "absolute",
							inset: 0,
							pointerEvents: "none",
							zIndex: 4,
						}}
					>
						{dragSelection.method === "rectangle" ? (
							<rect
								x={Math.min(dragOverlayPoints[0]?.x ?? 0, dragOverlayPoints[1]?.x ?? 0)}
								y={Math.min(dragOverlayPoints[0]?.y ?? 0, dragOverlayPoints[1]?.y ?? 0)}
								width={Math.abs((dragOverlayPoints[1]?.x ?? 0) - (dragOverlayPoints[0]?.x ?? 0))}
								height={Math.abs((dragOverlayPoints[1]?.y ?? 0) - (dragOverlayPoints[0]?.y ?? 0))}
								fill="rgba(102, 232, 255, 0.12)"
								stroke={dragSelection.coverage === "partial" ? "#66e8ff" : "#ffdf7a"}
								strokeDasharray={dragSelection.coverage === "partial" ? "5 4" : undefined}
								strokeWidth={1.5}
							/>
						) : (
							<polygon
								points={dragOverlayPoints.map((point) => `${point.x},${point.y}`).join(" ")}
								fill="rgba(102, 232, 255, 0.12)"
								stroke={dragSelection.coverage === "partial" ? "#66e8ff" : "#ffdf7a"}
								strokeDasharray={dragSelection.coverage === "partial" ? "5 4" : undefined}
								strokeWidth={1.5}
							/>
						)}
					</svg>
				) : null}
				{selectionMenu ? (
					<div
						onPointerDown={(e) => e.stopPropagation()}
						style={{
							position: "fixed",
							left: Math.min(selectionMenu.client.x + 8, window.innerWidth - 230),
							top: Math.min(selectionMenu.client.y + 8, window.innerHeight - 220),
							width: 220,
							maxHeight: 210,
							overflowY: "auto",
							background: "#10101a",
							border: "1px solid #4c5a78",
							borderRadius: 7,
							boxShadow: "0 10px 28px rgba(0,0,0,0.55)",
							zIndex: 10080,
							padding: 4,
						}}
					>
						<div style={{ fontSize: 11, opacity: 0.7, padding: "4px 6px" }}>Select target</div>
						{selectionMenu.targets.map((target) => {
							const key = spatialPickTargetKey(target);
							const active = hoveredPickKey === key;
							return (
								<button
									key={key}
									type="button"
									onPointerEnter={() =>
										setHoveredPickKey(effectiveSelectionKindToggles[target.kind] !== false ? key : null)
									}
									onPointerLeave={() => setHoveredPickKey(null)}
									onPointerDown={(e) => {
										e.preventDefault();
										e.stopPropagation();
										dispatchSelectionTargets([target], selectionMenu.modifiers, selectionMenu.point);
									}}
									style={{
										display: "block",
										width: "100%",
										border: "none",
										borderRadius: 5,
										padding: "6px 7px",
										textAlign: "left",
										background: active ? "#233b5d" : "transparent",
										color: "#e8e8f0",
										cursor: "pointer",
										fontSize: 12,
									}}
								>
									<span
										style={{
											display: "inline-block",
											width: 8,
											height: 8,
											borderRadius: 2,
											marginRight: 6,
											background: targetStyle(target, false, false).color,
										}}
									/>
									<span style={{ opacity: 0.7 }}>{target.kind}</span>{" "}
									<code style={{ color: "#ffffff" }}>{target.id}</code>
								</button>
							);
						})}
					</div>
				) : null}
			</div>
			{showAside ? (
			<aside
				style={{
					width: 360,
					padding: 12,
					background: "#12121c",
					borderLeft: "1px solid #2a2a3a",
					display: "flex",
					flexDirection: "column",
					gap: 10,
					overflow: "auto",
					position: "relative",
					zIndex: 2,
					...asideStyle,
				}}
			>
				<strong>Spatial play</strong>
				<div style={{ display: "flex", flexWrap: "wrap", gap: 6 }}>
					{transitionRows.map((row) => (
						<button
							key={`${row.key}-${row.eventKind}-${row.label}`}
							type="button"
							onClick={() => runTransitionRow(row)}
							style={{
								padding: "5px 7px",
								borderRadius: 6,
								border: "1px solid #2e3a52",
								background: "#182238",
								color: "#e8e8f0",
								cursor: "pointer",
								fontSize: 12,
							}}
						>
							<span style={{ textDecoration: "underline", fontWeight: 700 }}>{row.key}</span> {row.label}
						</button>
					))}
				</div>
				<div
					style={{
						display: "grid",
						position: "relative",
						overflow: "visible",
						borderRadius: 6,
						background: "#0e0e16",
						border: "1px solid #3a4762",
					}}
				>
					<input
						ref={cmdRef}
						type="text"
						autoComplete="off"
						spellCheck={false}
						value={cmdLine}
						onChange={(e) => {
							setCmdLine(replCommandTextWithoutSpaces(e.target.value));
							if (interactionMenuOpen) setInteractionMenuOpen(true);
						}}
						onKeyDown={onInputKeyDown}
						placeholder="Type an interaction or transition"
						style={{
							gridArea: "1 / 1",
							width: "100%",
							boxSizing: "border-box",
							padding: "8px 34px 8px 9px",
							borderRadius: 6,
							background: "transparent",
							color: "#e8e8f0",
							border: "none",
							outline: "none",
							fontSize: 13,
							fontFamily: "inherit",
							lineHeight: "normal",
						}}
					/>
					<button
						type="button"
						onMouseDown={(e) => e.preventDefault()}
						onClick={() => {
							setInteractionMenuOpen((open) => !open);
							cmdRef.current?.focus();
						}}
						aria-label="Show matching interactions"
						style={{
							gridArea: "1 / 1",
							justifySelf: "end",
							alignSelf: "center",
							marginRight: 6,
							width: 22,
							height: 22,
							borderRadius: 4,
							border: "1px solid #2e3a52",
							background: interactionMenuOpen ? "#1f3656" : "#141420",
							color: "#e8e8f0",
							cursor: "pointer",
							fontSize: 11,
							lineHeight: "20px",
							padding: 0,
							zIndex: 1,
						}}
					>
						v
					</button>
					{completionSuffix ? (
						<div
							aria-hidden
							style={{
								gridArea: "1 / 1",
								pointerEvents: "none",
								boxSizing: "border-box",
								padding: "8px 34px 8px 9px",
								fontSize: 13,
								fontFamily: "inherit",
								lineHeight: "normal",
								whiteSpace: "pre",
								overflow: "hidden",
								color: "#e8e8f0",
							}}
						>
							<span style={{ color: "transparent" }}>{cmdLine}</span>
							<span style={{ opacity: 0.45 }}>{completionSuffix}</span>
						</div>
					) : null}
					{interactionMenuOpen ? (
						<div
							onPointerDown={(e) => e.stopPropagation()}
							style={{
								position: "absolute",
								top: "calc(100% + 6px)",
								right: 0,
								width: 280,
								maxWidth: "calc(100vw - 32px)",
								maxHeight: 220,
								overflowY: "auto",
								background: "#10101a",
								border: "1px solid #4c5a78",
								borderRadius: 7,
								boxShadow: "0 10px 28px rgba(0,0,0,0.55)",
								zIndex: 3,
								padding: 4,
							}}
						>
							{interactionMatches.length ? (
								interactionMatches.map((suggestion) => (
									<button
										key={`${suggestion.kind}:${suggestion.key}:${suggestion.detail}`}
										type="button"
										onClick={() => runSuggestion(suggestion)}
										style={{
											display: "flex",
											flexDirection: "column",
											gap: 4,
											width: "100%",
											border: "none",
											borderRadius: 5,
											padding: "6px 7px",
											textAlign: "left",
											background: "transparent",
											color: "#e8e8f0",
											cursor: "pointer",
											fontSize: 12,
										}}
									>
										<div style={{ display: "flex", alignItems: "center", gap: 8 }}>
											<span
												style={{
													display: "inline-flex",
													alignItems: "center",
													justifyContent: "center",
													minWidth: 24,
													height: 20,
													padding: "0 6px",
													borderRadius: 999,
													border: "1px solid #2e3a52",
													background: "#182238",
													fontSize: 11,
													fontWeight: 700,
													textTransform: "uppercase",
												}}
											>
												{suggestion.key}
											</span>
											<span>{suggestion.label}</span>
										</div>
										<div style={{ fontSize: 11, opacity: 0.7 }}>{suggestion.detail}</div>
									</button>
								))
							) : (
								<div style={{ padding: "6px 7px", fontSize: 12, opacity: 0.7 }}>No matching interactions.</div>
							)}
						</div>
					) : null}
				</div>
				{asideExtra}
				{onDocumentModelChange ? (
					<SelectionAttributesPanel
						model={documentModel.model}
						activeModelDefinitionId={activeModelDefinitionId ?? GEOMETRY_MODEL_DEFINITION_ID}
						selection={displayedSelectionTargets}
						selectionCount={displayedSelectionTargets.length}
						onModelChange={onDocumentModelChange}
					/>
				) : null}
				<div style={{ display: "flex", flexDirection: "column", gap: 6, fontSize: 12 }}>
					{hideModelDefinitionControls ? null : (
						<>
							<label style={{ display: "flex", flexDirection: "column", gap: 4 }}>
								<span>Model definition</span>
								<select
									value={activeModelDefinitionId ?? GEOMETRY_MODEL_DEFINITION_ID}
									onChange={(e) => {
										const next = e.target.value || GEOMETRY_MODEL_DEFINITION_ID;
										setActiveModelDefinitionId(next);
										setModelDefinitionRevision((r) => r + 1);
										setSelectionMenu(null);
										setHoveredPickKey(null);
									}}
									style={{ padding: 6, borderRadius: 6, background: "#1a1a28", color: "#e8e8f0" }}
								>
									{modelDefinitions.map((row) => (
										<option key={row.id} value={row.id}>
											{row.label} ({row.id})
										</option>
									))}
								</select>
							</label>
							<span style={{ opacity: 0.75 }}>
								{modelDefinitionScope.typologies.length} typolog{modelDefinitionScope.typologies.length === 1 ? "y" : "ies"}
								{" · "}
								{modelDefinitionScope.interactions.length} interaction{modelDefinitionScope.interactions.length === 1 ? "" : "s"}
								{" · "}
								{modelDefinitionScope.attributeDefinitions.length} attribute{modelDefinitionScope.attributeDefinitions.length === 1 ? "" : "s"}
								{" · "}
								{modelDefinitionScope.propertyDefinitions.length} propert{modelDefinitionScope.propertyDefinitions.length === 1 ? "y" : "ies"}
							</span>
							{transformsFrom.length ? (
								<label style={{ display: "flex", flexDirection: "column", gap: 4 }}>
									<span>Transform from</span>
									<select
										defaultValue=""
										onChange={(e) => {
											const qid = e.target.value;
											if (!qid) return;
											const spec = transformsFrom.find((row) => qualifiedTransformationId(row.modelDefinitionId, row.id) === qid);
											if (spec) onApplyTransformation?.(spec);
											e.target.value = "";
										}}
										style={{ padding: 6, borderRadius: 6, background: "#1a1a28", color: "#e8e8f0" }}
									>
										<option value="">Select incoming transformation…</option>
										{transformsFrom.map((row) => (
											<option key={qualifiedTransformationId(row.modelDefinitionId, row.id)} value={qualifiedTransformationId(row.modelDefinitionId, row.id)}>
												{row.label} ({row.source.modelDefinition} → {row.target.modelDefinition})
											</option>
										))}
									</select>
								</label>
							) : null}
							{transformsTo.length ? (
								<label style={{ display: "flex", flexDirection: "column", gap: 4 }}>
									<span>Transform to</span>
									<select
										defaultValue=""
										onChange={(e) => {
											const qid = e.target.value;
											if (!qid) return;
											const spec = transformsTo.find((row) => qualifiedTransformationId(row.modelDefinitionId, row.id) === qid);
											if (spec) onApplyTransformation?.(spec);
											e.target.value = "";
										}}
										style={{ padding: 6, borderRadius: 6, background: "#1a1a28", color: "#e8e8f0" }}
									>
										<option value="">Select outgoing transformation…</option>
										{transformsTo.map((row) => (
											<option key={qualifiedTransformationId(row.modelDefinitionId, row.id)} value={qualifiedTransformationId(row.modelDefinitionId, row.id)}>
												{row.label} ({row.source.modelDefinition} → {row.target.modelDefinition})
											</option>
										))}
									</select>
								</label>
							) : null}
						</>
					)}
					{!isGeometryModelDefinition(activeModelDefinitionId) ? (
						<span style={{ opacity: 0.75 }}>
							{viewObjectCount} object{viewObjectCount === 1 ? "" : "s"}
						</span>
					) : null}
					<span>Selection method</span>
					<div role="group" aria-label="Selection method" style={{ display: "flex", gap: 6 }}>
						{(["rectangle", "lasso"] as const).map((method) => {
							const active = selectionMethod === method;
							return (
								<button
									key={method}
									type="button"
									onClick={() => setSelectionMethod(method)}
									style={{
										padding: "5px 8px",
										borderRadius: 999,
										border: active ? "1px solid #77aaff" : "1px solid #2a2a3a",
										background: active ? "#1f3656" : "#12121c",
										color: "#e8e8f0",
										cursor: "pointer",
										fontSize: 12,
										textTransform: "capitalize",
									}}
								>
									{method}
								</button>
							);
						})}
					</div>
					<span>Show kinds</span>
					<div role="group" aria-label="Show kinds" style={{ display: "flex", flexWrap: "wrap", gap: 6 }}>
						{activePickKinds.map((kind) => (
							<label
								key={`filter-${kind}`}
								style={{
									display: "flex",
									alignItems: "center",
									gap: 4,
									padding: "3px 6px",
									border: "1px solid #2a2a3a",
									borderRadius: 999,
									background: filterKindToggles[kind] ? "#1a3040" : "#12121c",
								}}
							>
								<input
									type="checkbox"
									checked={filterKindToggles[kind]}
									onChange={(e) => {
										setFilterKindToggles((prev) => ({ ...prev, [kind]: e.target.checked }));
									}}
								/>
								{kind}
							</label>
						))}
					</div>

					<span>Selection kinds</span>
					<div role="group" aria-label="Selection kinds" style={{ display: "flex", flexWrap: "wrap", gap: 6 }}>
						{activePickKinds.map((kind) => {
							const accepted = pickKindEnabledForSelectionAccept(activeSelectionAccept, kind);
							return (
								<label
									key={`select-${kind}`}
									style={{
										display: "flex",
										alignItems: "center",
										gap: 4,
										padding: "3px 6px",
										border: "1px solid #2a2a3a",
										borderRadius: 999,
										opacity: accepted ? 1 : 0.45,
										background: selectionKindToggles[kind] ? "#1a2638" : "#12121c",
									}}
								>
									<input
										type="checkbox"
										checked={selectionKindToggles[kind]}
										onChange={(e) => {
											const checked = e.target.checked;
											setSelectionKindToggles((prev) => ({ ...prev, [kind]: checked }));
											setSelectionMenu(null);
											setHoveredPickKey(null);
											if (!checked) applySelectionPrune((prev) => replPruneSelectionByKind(prev, activeModelDefinitionId, kind));
										}}
									/>
									{kind}
								</label>
							);
						})}
					</div>
					{modelDefinitionScope.selectionOperations.length ? (
						<>
							<span>Selection commands</span>
							<div role="group" aria-label="Selection commands" style={{ display: "flex", flexWrap: "wrap", gap: 6 }}>
								{modelDefinitionScope.selectionOperations.map((defn) => (
									<button
										key={defn.id}
										type="button"
										title={defn.id}
										onClick={() => {
											void rt.query(`CALL ${defn.id}({}) YIELD data.targets AS targets`);
										}}
										style={{
											padding: "4px 8px",
											borderRadius: 999,
											border: "1px solid #2a2a3a",
											background: "#12121c",
											color: "#e8e8f0",
											cursor: "pointer",
											fontSize: 12,
										}}
									>
										{defn.label}
									</button>
								))}
							</div>
						</>
					) : null}
				</div>
				<div style={{ fontSize: 12, opacity: 0.85 }}>
					{interactionId ? (
						<>
							Interaction <code>{interactionId}</code> ┬À state <code>{snapshot.state}</code> ┬À rev {snapshot.revision}
						</>
					) : (
						<>
							No interaction selected ┬À state <code>{snapshot.state}</code> ┬À rev {snapshot.revision}
						</>
					)}
				</div>
				<div style={{ fontSize: 12, borderTop: "1px solid #2a2a3a", paddingTop: 8 }}>
					<strong>Last response</strong>
					<pre style={{ fontSize: 10, overflow: "auto", maxHeight: 120, margin: "6px 0 0" }}>
						{lr ? JSON.stringify(lr, null, 2) : "ÔÇö"}
					</pre>
					{snapshot.diagnostics.length ? (
						<ul style={{ fontSize: 11, margin: 0, paddingLeft: 16 }}>
							{snapshot.diagnostics.map((d, i) => (
								<li key={`${d.code}-${i}`}>
									[{d.severity}] {d.code}: {d.message}
								</li>
							))}
						</ul>
					) : null}
				</div>
			</aside>
			) : null}
		</div>
	);
}

/** @emoji ­ƒ╝´©Å Canvas-only {@link InteractionRepl} (no built-in aside); full host props and `on*` callbacks. */
export function InteractionReplViewport(props: InteractionReplProps): ReactNode {
	return <InteractionRepl {...props} showAside={false} />;
}

export interface SelectionAttributesPanelProps {
	readonly model: Model;
	readonly activeModelDefinitionId: string;
	readonly selection: readonly SelectionTarget[];
	readonly selectionCount?: number;
	readonly onModelChange: (model: Model) => void;
}

const ATTRIBUTE_FIELD_STYLE = { padding: 6, borderRadius: 6, background: "#1a1a28", color: "#e8e8f0", border: "1px solid #2a2a3c" } as const;

/** @emoji 🏷️ Edits {@link Model.metadata} fields for the primary selection using active model-definition attribute assets. */
export function SelectionAttributesPanel({
	model,
	activeModelDefinitionId,
	selection,
	selectionCount,
	onModelChange,
}: SelectionAttributesPanelProps): ReactNode {
	const target = useMemo(() => primaryAttributeSelectionTarget(selection), [selection]);
	const definitions = useMemo(
		() => (target ? listAttributeDefinitionsForModelDefinitionEntity(activeModelDefinitionId, target.kind) : []),
		[activeModelDefinitionId, target],
	);
	if (!target || !definitions.length) return null;
	const fields = model.metadata.get(target.id) ?? {};
	const count = selectionCount ?? selection.length;
	const setField = (defn: AttributeDefinitionSpec, value: unknown) => {
		if (!validateAttributeValue(defn, value)) return;
		model.metadata.setField(target.id, defn.field, value);
		onModelChange(model);
	};
	return (
		<div style={{ display: "flex", flexDirection: "column", gap: 6, fontSize: 12 }}>
			<span style={{ fontWeight: 600, color: "#c8c8e0" }}>Attributes</span>
			<span style={{ opacity: 0.75, fontSize: 11 }}>
				{target.kind} · <code style={{ color: "#e8e8f0" }}>{target.id}</code>
				{count > 1 ? ` · ${count} selected` : ""}
			</span>
			{definitions.map((defn) => {
				const editor = attributeDefinitionEditorKind(defn);
				const current = fields[defn.field];
				if (editor === "enum") {
					const options = attributeDefinitionValueOptions(defn) ?? [];
					return (
						<label key={defn.id} style={{ display: "flex", flexDirection: "column", gap: 4 }}>
							<span>{defn.label}</span>
							<select
								value={typeof current === "string" ? current : ""}
								onChange={(e) => setField(defn, e.target.value)}
								style={ATTRIBUTE_FIELD_STYLE}
							>
								<option value="">—</option>
								{options.map((option) => (
									<option key={option} value={option}>
										{option}
									</option>
								))}
							</select>
						</label>
					);
				}
				if (editor === "number") {
					return (
						<label key={defn.id} style={{ display: "flex", flexDirection: "column", gap: 4 }}>
							<span>{defn.label}</span>
							<input
								type="number"
								value={typeof current === "number" ? current : ""}
								onChange={(e) => setField(defn, e.target.value === "" ? 0 : Number(e.target.value))}
								style={ATTRIBUTE_FIELD_STYLE}
							/>
						</label>
					);
				}
				if (editor === "boolean") {
					return (
						<label key={defn.id} style={{ display: "flex", alignItems: "center", gap: 6 }}>
							<input type="checkbox" checked={current === true} onChange={(e) => setField(defn, e.target.checked)} />
							<span>{defn.label}</span>
						</label>
					);
				}
				return (
					<label key={defn.id} style={{ display: "flex", flexDirection: "column", gap: 4 }}>
						<span>{defn.label}</span>
						<input
							type="text"
							value={typeof current === "string" ? current : current === undefined || current === null ? "" : JSON.stringify(current)}
							onChange={(e) => setField(defn, e.target.value)}
							style={ATTRIBUTE_FIELD_STYLE}
						/>
					</label>
				);
			})}
		</div>
	);
}

export interface SelectionPropertiesPanelProps {
	readonly model: Model;
	readonly kernel: SpatialKernel;
	readonly activeModelDefinitionId: string;
	readonly selection: readonly SelectionTarget[];
	readonly selectionCount?: number;
}

/** @emoji 📐 Displays derived property values for the primary object selection using scoped property definitions. */
export function SelectionPropertiesPanel({
	model,
	kernel,
	activeModelDefinitionId,
	selection,
	selectionCount,
}: SelectionPropertiesPanelProps): ReactNode {
	const objectRow = useMemo(() => {
		const objectTarget = selection.find((row) => row.kind === "object");
		return objectTarget ? (model.objects[objectTarget.id] ?? null) : null;
	}, [model, selection]);
	const definitions = useMemo(
		() => (objectRow ? listApplicablePropertyDefinitionsForModelDefinition(activeModelDefinitionId, model, objectRow) : []),
		[activeModelDefinitionId, model, objectRow],
	);
	const [values, setValues] = useState<Readonly<Record<string, Record<string, unknown>>>>({});
	useEffect(() => {
		if (!objectRow || !definitions.length) {
			setValues({});
			return;
		}
		let cancelled = false;
		void (async () => {
			const next: Record<string, Record<string, unknown>> = {};
			for (const defn of definitions) {
				next[defn.id] = await derivePropertyValue(defn, { model, kernel, object: objectRow });
			}
			if (!cancelled) setValues(next);
		})();
		return () => {
			cancelled = true;
		};
	}, [definitions, kernel, model, objectRow]);
	if (!objectRow || !definitions.length) return null;
	const count = selectionCount ?? selection.length;
	return (
		<div style={{ display: "flex", flexDirection: "column", gap: 6, fontSize: 12 }}>
			<span style={{ fontWeight: 600, color: "#c8c8e0" }}>Properties</span>
			<span style={{ opacity: 0.75, fontSize: 11 }}>
				object · <code style={{ color: "#e8e8f0" }}>{objectRow.id}</code>
				{count > 1 ? ` · ${count} selected` : ""}
			</span>
			{definitions.map((defn) => (
				<div key={defn.id} style={{ display: "flex", flexDirection: "column", gap: 2 }}>
					<span>{defn.label}</span>
					<pre style={{ margin: 0, fontSize: 11, opacity: 0.85, overflow: "auto" }}>{JSON.stringify(values[defn.id] ?? {}, null, 2)}</pre>
				</div>
			))}
		</div>
	);
}
// #endregion ­ƒ¬®Repl

// #region ­ƒº¬Tests
const __spatialR3fTestKernel = import.meta.vitest ? await import("@spatial/js-kernel-brepjs") : null;

if (import.meta.vitest) {
	const { BrepjsKernel, preciseSpatialKernelMath: M } = __spatialR3fTestKernel!;
	const { describe, it, expect } = import.meta.vitest;

	describe("@spatial/js-renderer-r3f interaction adapter", () => {
		it("keeps active spatial ground picks enabled when host geometry selection is disabled", () => {
			const snapshot = {
				state: "first_corner",
				spatialInteraction: {
					spatialGroundPick: true,
					pickDisabledStates: ["idle", "ready", "committed"],
					groundPointerMoveStates: ["first_corner"],
					heightDragStates: [],
					verticalRodStates: [],
					heightConfirmState: null,
				},
			} satisfies Pick<InteractionSnapshot, "state" | "spatialInteraction">;
			expect(interactionSpatialGroundPickPlaneEnabled(snapshot, true, [])).toBe(true);
			expect(interactionSpatialGroundPickPlaneEnabled(snapshot, true, ["vertex"])).toBe(false);
		});

		it("creates snap and selection metadata for geometry targets", () => {
			const targets = createSpatialPickTargets({
				schema: "spatial.model/v1",
				revision: 1,
				anchors: [],
				vertices: [{ id: "v0", position: [1, 2, 3] }],
				edges: [],
				wires: [],
				faces: [],
				shells: [],
				solids: [],
			} as unknown as ModelJson);
			expect(targets).toEqual([{ kind: "vertex", geometryKind: "vertex", id: "v0", point: [1, 2, 3] }]);
			expect(createSpatialPickEvent("pointer.down", [9, 9, 9], targets[0]!, { shift: true })).toEqual({
				kind: "pointer.down",
				point: [9, 9, 9],
				modifiers: { shift: true },
				snap: { kind: "vertex", id: "v0", point: [1, 2, 3] },
				selection: { kind: "vertex", id: "v0" },
			});
		});

		it("adds typology object picks for non-builtin model definitions", async () => {
			const model = new Model();
			const cell = solidRef("c0");
			applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, cell));
			model.objects["object-c0"] = {
				id: "object-c0" as ObjectRef,
				typology: "energy.energy.hull",
				primitives: { solid: String(cell) },
			};
			const activeModelDefinitionId = "aec.building.energy";
			const editTargets = createSpatialPickTargets(model, GEOMETRY_MODEL_DEFINITION_ID);
			const objectTargets = createSpatialPickTargets(model, activeModelDefinitionId);
			expect(editTargets.some((t) => t.kind === "vertex")).toBe(true);
			expect(objectTargets.every((t) => t.kind === "object")).toBe(true);
			expect(objectTargets.length).toBeGreaterThan(0);
			const structureTargets = createSpatialPickTargets(model, "aec.building.structure");
			expect(structureTargets.some((t) => t.kind === "face")).toBe(true);
			expect(structureTargets.some((t) => t.kind === "object")).toBe(true);
		});

		it("filterSpatialPickTargetsForActiveView scopes by model definition entity kinds", () => {
			const targets: SpatialPickTarget[] = [
				{ kind: "vertex", geometryKind: "vertex", id: "v0", point: [0, 0, 0] },
				{ kind: "face", geometryKind: "face", id: "f0", point: [0.5, 0.5, 0.5] },
				{ kind: "object", id: "energy.energy.hull", point: [0.5, 0.5, 0.5] },
			];
			expect(filterSpatialPickTargetsForActiveView(targets, GEOMETRY_MODEL_DEFINITION_ID).map(spatialPickTargetKey)).toEqual([
				"vertex:v0",
				"face:f0",
			]);
			expect(filterSpatialPickTargetsForActiveView(targets, "aec.building.energy").map(spatialPickTargetKey)).toEqual([
				"object:energy.energy.hull",
			]);
			expect(filterSpatialPickTargetsForActiveView(targets, "aec.building.structure").map(spatialPickTargetKey)).toEqual([
				"face:f0",
				"object:energy.energy.hull",
			]);
		});

		it("resolveSpatialSceneVisibility switches edit wireframe vs committed object mesh", () => {
			expect(resolveSpatialSceneVisibility(GEOMETRY_MODEL_DEFINITION_ID, { edge: true, face: true })).toEqual({
				showFactoryWireframe: true,
				showCommittedFaces: true,
				showCommittedEdges: true,
			});
			expect(resolveSpatialSceneVisibility("aec.building.energy", { object: true })).toEqual({
				showFactoryWireframe: false,
				showCommittedFaces: false,
				showCommittedEdges: false,
			});
		});

		it("defaultInteractionReplChromeState seeds builtin geometry edit by default", () => {
			const chrome = defaultInteractionReplChromeState();
			expect(chrome.activeModelDefinitionId).toBe(GEOMETRY_MODEL_DEFINITION_ID);
			expect(chrome.filterKindToggles.vertex).toBe(true);
		});

		it("scopes displayed selection to activeModelDefinitionId", () => {
			const renderer = [
				{ kind: "face", id: "f0", editable: true },
				{ kind: "object", id: "o0", editable: false },
			] satisfies readonly SelectionTarget[];
			expect(replDisplayedSelectionTargets(false, GEOMETRY_MODEL_DEFINITION_ID, renderer, [])).toEqual([{ kind: "face", id: "f0", editable: true }]);
			expect(replDisplayedSelectionTargets(false, "aec.building.energy", renderer, [])).toEqual([
				{ kind: "object", id: "o0", editable: false },
			]);
			expect(replDisplayedSelectionTargets(false, "aec.building.structure", renderer, [])).toEqual([
				{ kind: "face", id: "f0", editable: true },
				{ kind: "object", id: "o0", editable: false },
			]);
		});

		it("creates anchor and shell pick targets for builtin geometry", () => {
			const model = new Model();
			model.anchors["a0"] = { id: "a0" as AnchorRef, position: [0, 0, 0], attachment: { kind: "vertex", id: "v0" } };
			model.vertices["v0"] = { id: "v0" as VertexRef, position: [0, 0, 0] };
			const cell = solidRef("c0");
			applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, cell));
			const faceId = Object.keys(model.faces)[0]!;
			model.shells["sh0"] = { id: "sh0" as ShellRef, faceIds: [faceId] };
			const targets = createSpatialPickTargets(model, GEOMETRY_MODEL_DEFINITION_ID);
			expect(targets.some((t) => t.geometryKind === "anchor")).toBe(true);
			expect(targets.some((t) => t.geometryKind === "shell")).toBe(true);
		});

		it("modelDefinitionPickTargetKinds maps topology entity kinds to pick toggles", () => {
			expect(modelDefinitionPickTargetKinds(GEOMETRY_MODEL_DEFINITION_ID).sort()).toEqual(["edge", "face", "object", "vertex"]);
			expect(modelDefinitionPickTargetKinds("aec.building.structure").sort()).toEqual(["edge", "face", "object"]);
		});

		it("merges picks within active model definition without clearing out-of-scope targets", () => {
			const renderer: SelectionTarget[] = [
				{ kind: "wire", id: "w0", editable: true },
				{ kind: "object", id: "o0", editable: false },
			];
			expect(
				replMergeSelectionPickInView(false, GEOMETRY_MODEL_DEFINITION_ID, renderer, [], [{ kind: "wire", id: "w1", editable: true }], {}),
			).toEqual([
				{ kind: "object", id: "o0", editable: false },
				{ kind: "wire", id: "w1", editable: true },
			]);
		});
	});
}