import {
	bsplineApprox,
	compound,
	face as brepFace,
	getKernel,
	initFromOC,
	line,
	mesh,
	meshEdges,
	polygon,
	solid,
	toBufferGeometryData,
	toLineGeometryData,
	unwrap,
	wire,
	wireLoop,
	sewShells,
	type BufferGeometryData,
	type LineGeometryData,
} from "brepjs";
import createOpenCascade from "brepjs-opencascade/src/brepjs_single.js";
import openCascadeWasmUrl from "brepjs-opencascade/src/brepjs_single.wasm?url";

//#region 🔖Kinds
export const TOPOLOGIC_KINDS = ["topology", "vertex", "edge", "wire", "face", "shell", "cell", "cellComplex", "cluster"] as const;

export type TopologicKind = (typeof TOPOLOGIC_KINDS)[number];
export type Vec3 = readonly [number, number, number];
export type Quat = readonly [number, number, number, number];

export interface TopologicTransform {
	readonly position?: Vec3;
	readonly rotation?: Quat;
	readonly scale?: number | Vec3;
}

export interface TopologicStyle {
	readonly color?: string;
	readonly edgeColor?: string;
	readonly opacity?: number;
	readonly lineWidth?: number;
	readonly pointSize?: number;
}

interface TopologicEntityBase {
	readonly id: string;
	readonly kind: TopologicKind;
	readonly label?: string;
	readonly description?: string;
	readonly style?: TopologicStyle;
	readonly transform?: TopologicTransform;
	readonly metadata?: Record<string, unknown>;
}

export interface TopologicTopologyEntity extends TopologicEntityBase {
	readonly kind: "topology";
	readonly members: readonly string[];
}

export interface TopologicVertexEntity extends TopologicEntityBase {
	readonly kind: "vertex";
	readonly point: Vec3;
	readonly radius?: number;
}

export interface TopologicEdgeEntity extends TopologicEntityBase {
	readonly kind: "edge";
	readonly vertices: readonly [string, string];
	readonly curve?: readonly Vec3[];
}

export interface TopologicWireEntity extends TopologicEntityBase {
	readonly kind: "wire";
	readonly edges: readonly string[];
	readonly closed?: boolean;
	readonly manifold?: boolean;
}

export interface TopologicFaceEntity extends TopologicEntityBase {
	readonly kind: "face";
	readonly wires: readonly string[];
	readonly surface: {
		readonly vertices: readonly Vec3[];
		readonly triangles: readonly number[];
	};
}

export interface TopologicShellEntity extends TopologicEntityBase {
	readonly kind: "shell";
	readonly faces: readonly string[];
}

export interface TopologicCellEntity extends TopologicEntityBase {
	readonly kind: "cell";
	readonly shells: readonly string[];
}

export interface TopologicCellComplexEntity extends TopologicEntityBase {
	readonly kind: "cellComplex";
	readonly cells: readonly string[];
}

export interface TopologicClusterEntity extends TopologicEntityBase {
	readonly kind: "cluster";
	readonly topologies: readonly string[];
}

export type TopologicEntity =
	| TopologicTopologyEntity
	| TopologicVertexEntity
	| TopologicEdgeEntity
	| TopologicWireEntity
	| TopologicFaceEntity
	| TopologicShellEntity
	| TopologicCellEntity
	| TopologicCellComplexEntity
	| TopologicClusterEntity;

export interface TopologicFixtureV1 {
	readonly schema: "elements.geometry.topologic.fixture/v1";
	readonly label?: string;
	readonly roots: readonly string[];
	readonly topologies: readonly TopologicEntity[];
}

export interface TopologicRenderPacketEntryV1 {
	readonly id: string;
	readonly kind: TopologicKind;
	readonly position: Float32Array;
	readonly rotation: Float32Array;
	readonly scale: Float32Array;
	readonly points?: Float32Array;
	readonly triangles?: Uint32Array;
}

export interface TopologicRenderPacketV1 {
	readonly entries: readonly TopologicRenderPacketEntryV1[];
	readonly revisitedIds: readonly string[];
}

export interface SpatialRenderable {
	readonly id: string;
	readonly kind: TopologicKind;
	readonly label: string;
	readonly style: TopologicStyle | undefined;
	readonly transform: TopologicTransform | undefined;
	readonly fill?: BufferGeometryData;
	readonly edges?: LineGeometryData;
	readonly point?: { readonly position: Vec3; readonly radius: number };
	readonly children?: readonly SpatialRenderable[];
}

export type SpatialStatus = "loading" | "ready" | "error";
export type SpatialSurfaceKindFilter = TopologicKind | "all";

export interface SpatialPanelKindOption {
	readonly kind: SpatialSurfaceKindFilter;
	readonly label: string;
	readonly active: boolean;
}

export interface SpatialWorkbenchPanelEntry {
	readonly id: string;
	readonly label: string;
	readonly kindLabel: string;
	readonly selected: boolean;
}

export interface SpatialWorkbenchPanelState {
	readonly fixtureLabel?: string;
	readonly visibleKindsLabel: string;
	readonly selectableKindsLabel: string;
	readonly query: string;
	readonly focusOptions: readonly SpatialPanelKindOption[];
	readonly entityCount: number;
	readonly entities: readonly SpatialWorkbenchPanelEntry[];
	readonly setFocusedKind: (kind: SpatialSurfaceKindFilter) => void;
	readonly setSelectedId: (id: string | null) => void;
	readonly setQuery: (query: string) => void;
}

export interface SpatialDetailsPanelState {
	readonly selectedLabel: string;
	readonly selectedKindLabel: string;
	readonly description?: string;
	readonly status: SpatialStatus;
	readonly focusedKindLabel: string;
	readonly query: string;
}

export interface SpatialSurfaceSnapshot {
	readonly status: SpatialStatus;
	readonly fixtureLabel?: string;
	readonly model: SpatialModel | null;
	readonly focusedKind: SpatialSurfaceKindFilter;
	readonly selectedId: string | null;
	readonly query: string;
	readonly error: string | null;
	readonly selectableKinds: Readonly<Record<TopologicKind, boolean>>;
	readonly visibleKinds: Readonly<Record<TopologicKind, boolean>>;
	readonly setSelectedId: (id: string | null) => void;
	readonly setEntityTransform: (id: string, transform: TopologicTransform) => void;
	readonly workbenchPanel: SpatialWorkbenchPanelState;
	readonly detailsPanel: SpatialDetailsPanelState;
}

export interface SpatialJsBindings {
	readonly parseFixture: (raw: unknown) => TopologicFixtureV1 | null;
	readonly deriveAnalyzeFixture: (fixture: TopologicFixtureV1) => TopologicFixtureV1 | null;
	readonly buildRenderPacket: (fixture: TopologicFixtureV1) => TopologicRenderPacketV1 | null;
	readonly vertexPoint: (fixture: TopologicFixtureV1, id: string) => Vec3 | null;
	readonly edgeCurve: (fixture: TopologicFixtureV1, id: string) => readonly Vec3[];
	readonly updateFixtureTransform: (fixture: TopologicFixtureV1, entityId: string, transform: TopologicTransform) => TopologicFixtureV1 | null;
}

export interface TopologicWasmBindings extends SpatialJsBindings {}
//#endregion 🔖Kinds

//#region 🔖Kernel
let kernelPromise: Promise<void> | null = null;
let spatialBindings: SpatialJsBindings | null = null;
let spatialBindingsPromise: Promise<SpatialJsBindings> | null = null;

function stripWindowsDriveSlash(pathname: string): string {
	return /^\/[A-Za-z]:/.test(pathname) ? pathname.slice(1) : pathname;
}

function fileUrlToPathname(fileUrl: string): string {
	return stripWindowsDriveSlash(decodeURIComponent(new URL(fileUrl).pathname));
}

function resolveOpenCascadeWasmLocation(): string {
	const isNodeLike = typeof process !== "undefined" && typeof process.versions?.node === "string";
	if (openCascadeWasmUrl.startsWith("file:///")) {
		const pathname = fileUrlToPathname(openCascadeWasmUrl);
		if (isNodeLike) {
			return pathname.replaceAll("/", "\\");
		}
		return `/@fs/${pathname.replaceAll("\\", "/")}`;
	}
	if (openCascadeWasmUrl.startsWith("/@fs/")) {
		const fsPath = openCascadeWasmUrl.slice("/@fs/".length);
		return isNodeLike ? fsPath.replaceAll("/", "\\") : openCascadeWasmUrl;
	}
	return openCascadeWasmUrl;
}

/** @emoji ⚙️ Ensures the brepjs kernel is initialized once for the spatial adapter. */
export function ensureSpatialKernelLoaded(): Promise<void> {
	kernelPromise ??= (async () => {
		try {
			getKernel();
			return;
		} catch {
			const wasmLocation = resolveOpenCascadeWasmLocation();
			const oc = await createOpenCascade({
				locateFile: (fileName: string) => (fileName.endsWith(".wasm") ? wasmLocation : fileName),
			});
			initFromOC(oc);
		}
	})();
	return kernelPromise;
}

export function getSpatialJsBindings(): SpatialJsBindings {
	if (!spatialBindings) {
		throw new Error("Spatial kernel is not loaded yet. Call ensureSpatialJsBindingsLoaded() before using synchronous geometry helpers.");
	}
	return spatialBindings;
}

export async function ensureSpatialJsBindingsLoaded(): Promise<SpatialJsBindings> {
	spatialBindingsPromise ??= ensureSpatialKernelLoaded().then(() => {
		spatialBindings ??= {
			parseFixture: parseTopologicFixtureV1,
			deriveAnalyzeFixture: deriveAnalyzeTopologicFixtureV1,
			buildRenderPacket: buildTopologicRenderPacketV1,
			vertexPoint: vertexPointTopologicFixtureV1,
			edgeCurve: edgeCurveTopologicFixtureV1,
			updateFixtureTransform: updateTopologicFixtureTransformKernelV1,
		};
		return spatialBindings;
	});
	return spatialBindingsPromise;
}
//#endregion 🔖Kernel

//#region 🔖Parsing
function isVec3(value: unknown): value is Vec3 {
	return Array.isArray(value) && value.length === 3 && value.every((entry) => typeof entry === "number" && Number.isFinite(entry));
}

function isQuat(value: unknown): value is Quat {
	return Array.isArray(value) && value.length === 4 && value.every((entry) => typeof entry === "number" && Number.isFinite(entry));
}

function isTransform(value: unknown): value is TopologicTransform {
	if (!value || typeof value !== "object") return false;
	const candidate = value as Record<string, unknown>;
	if (candidate.position !== undefined && !isVec3(candidate.position)) return false;
	if (candidate.rotation !== undefined && !isQuat(candidate.rotation)) return false;
	if (
		candidate.scale !== undefined &&
		typeof candidate.scale !== "number" &&
		!isVec3(candidate.scale)
	) return false;
	return true;
}

function isEntity(value: unknown): value is TopologicEntity {
	if (!value || typeof value !== "object") return false;
	const candidate = value as Record<string, unknown>;
	if (typeof candidate.id !== "string" || typeof candidate.kind !== "string") return false;
	if (!TOPOLOGIC_KINDS.includes(candidate.kind as TopologicKind)) return false;
	if (candidate.transform !== undefined && !isTransform(candidate.transform)) return false;
	if (candidate.kind === "vertex") return isVec3(candidate.point);
	if (candidate.kind === "edge") return Array.isArray(candidate.vertices) && candidate.vertices.length === 2 && candidate.vertices.every((entry) => typeof entry === "string");
	if (candidate.kind === "wire") return Array.isArray(candidate.edges) && candidate.edges.every((entry) => typeof entry === "string");
	if (candidate.kind === "face") {
		return (
			Array.isArray(candidate.wires) &&
			candidate.wires.every((entry) => typeof entry === "string") &&
			candidate.surface != null &&
			typeof candidate.surface === "object" &&
			Array.isArray((candidate.surface as { vertices?: unknown }).vertices) &&
			Array.isArray((candidate.surface as { triangles?: unknown }).triangles)
		);
	}
	if (candidate.kind === "shell") return Array.isArray(candidate.faces) && candidate.faces.every((entry) => typeof entry === "string");
	if (candidate.kind === "cell") return Array.isArray(candidate.shells) && candidate.shells.every((entry) => typeof entry === "string");
	if (candidate.kind === "cellComplex") return Array.isArray(candidate.cells) && candidate.cells.every((entry) => typeof entry === "string");
	if (candidate.kind === "cluster") return Array.isArray(candidate.topologies) && candidate.topologies.every((entry) => typeof entry === "string");
	if (candidate.kind === "topology") return Array.isArray(candidate.members) && candidate.members.every((entry) => typeof entry === "string");
	return false;
}

function fixtureEntityChildIds(entity: TopologicEntity): readonly string[] {
	if (entity.kind === "topology") return entity.members;
	if (entity.kind === "wire") return entity.edges;
	if (entity.kind === "face") return entity.wires;
	if (entity.kind === "shell") return entity.faces;
	if (entity.kind === "cell") return entity.shells;
	if (entity.kind === "cellComplex") return entity.cells;
	if (entity.kind === "cluster") return entity.topologies;
	if (entity.kind === "edge") return entity.vertices;
	return [];
}

function hasResolvableFixtureReferences(fixture: TopologicFixtureV1): boolean {
	const ids = new Set(fixture.topologies.map((entity) => entity.id));
	if (!fixture.roots.every((id) => ids.has(id))) return false;
	return fixture.topologies.every((entity) => fixtureEntityChildIds(entity).every((id) => ids.has(id)));
}

/** @emoji 📦 Parses a Topologic-compatible fixture for the spatial adapter. */
export function parseTopologicFixtureV1(raw: unknown): TopologicFixtureV1 | null {
	if (!raw || typeof raw !== "object") return null;
	const candidate = raw as Record<string, unknown>;
	if (candidate.schema !== "elements.geometry.topologic.fixture/v1") return null;
	if (!Array.isArray(candidate.roots) || !candidate.roots.every((entry) => typeof entry === "string")) return null;
	if (!Array.isArray(candidate.topologies) || !candidate.topologies.every(isEntity)) return null;
	const fixture = candidate as TopologicFixtureV1;
	return hasResolvableFixtureReferences(fixture) ? fixture : null;
}

/** @emoji 📥 Loads the spatial fixture contract after the kernel is ready. */
export async function loadTopologicFixtureV1(raw: unknown): Promise<TopologicFixtureV1 | null> {
	await ensureSpatialKernelLoaded();
	return parseTopologicFixtureV1(raw);
}

export function vertexPointTopologicFixtureV1(fixture: TopologicFixtureV1, id: string): Vec3 | null {
	const entity = fixture.topologies.find((candidate) => candidate.id === id);
	return entity?.kind === "vertex" ? entity.point : null;
}

export function edgeCurveTopologicFixtureV1(fixture: TopologicFixtureV1, id: string): readonly Vec3[] {
	const entity = fixture.topologies.find((candidate) => candidate.id === id);
	if (!entity || entity.kind !== "edge") return [];
	if (entity.curve && entity.curve.length >= 2) return entity.curve;
	const start = vertexPointTopologicFixtureV1(fixture, entity.vertices[0]);
	const end = vertexPointTopologicFixtureV1(fixture, entity.vertices[1]);
	return start && end ? [start, end] : [];
}

export function deriveAnalyzeTopologicFixtureV1(fixture: TopologicFixtureV1): TopologicFixtureV1 | null {
	if (!hasResolvableFixtureReferences(fixture)) return null;
	return {
		...fixture,
		topologies: fixture.topologies.map((entity) =>
			entity.kind === "edge" && (!entity.curve || entity.curve.length < 2)
				? { ...entity, curve: edgeCurveTopologicFixtureV1(fixture, entity.id) }
				: entity,
		),
	};
}

/** @emoji 🔁 Returns an immutable fixture clone with one entity transform replaced. */
export function updateTopologicFixtureTransformV1(
	fixture: TopologicFixtureV1,
	entityId: string,
	transform: TopologicTransform,
): TopologicFixtureV1 | null {
	if (!fixture.topologies.some((entity) => entity.id === entityId)) return null;
	return {
		...fixture,
		topologies: fixture.topologies.map((entity) => (entity.id === entityId ? { ...entity, transform } : entity)),
	};
}

export function updateTopologicFixtureTransformKernelV1(
	fixture: TopologicFixtureV1,
	entityId: string,
	transform: TopologicTransform,
): TopologicFixtureV1 | null {
	return updateTopologicFixtureTransformV1(fixture, entityId, transform);
}
//#endregion 🔖Parsing

//#region 🔖Helpers
function entityLabel(entity: TopologicEntity): string {
	return entity.label ?? entity.id;
}

function transformScale(transform: TopologicTransform | undefined): Vec3 {
	const scale = transform?.scale;
	if (Array.isArray(scale)) return scale;
	if (typeof scale === "number") return [scale, scale, scale];
	return [1, 1, 1];
}

function cloneStyle(style: TopologicStyle | undefined): TopologicStyle | undefined {
	return style ? { ...style } : undefined;
}

function flattenPolyline(points: readonly Vec3[], closed = false): Float32Array {
	if (points.length < 2) return new Float32Array(0);
	const pairs: number[] = [];
	for (let index = 0; index < points.length - 1; index += 1) {
		pairs.push(...points[index], ...points[index + 1]);
	}
	if (closed) pairs.push(...points[points.length - 1], ...points[0]);
	return new Float32Array(pairs);
	}

function shapeLines(shape: object): LineGeometryData | undefined {
	try {
		return toLineGeometryData(meshEdges(shape as never, { tolerance: 0.1, angularTolerance: 0.1, cache: true }));
	} catch {
		return undefined;
	}
}

function shapeFill(shape: object): BufferGeometryData | undefined {
	try {
		return toBufferGeometryData(mesh(shape as never, { tolerance: 0.1, angularTolerance: 0.1, cache: true }));
	} catch {
		return undefined;
	}
}

function faceSurfaceFill(face: TopologicFaceEntity): BufferGeometryData {
	return {
		position: new Float32Array(face.surface.vertices.flat()),
		normal: new Float32Array(0),
		index: new Uint32Array(face.surface.triangles),
	};
	}

type BrepShape = object;

function topologicTransformPacket(transform: TopologicTransform | undefined): {
	readonly position: Float32Array;
	readonly rotation: Float32Array;
	readonly scale: Float32Array;
} {
	const props = transformProps(transform);
	return {
		position: new Float32Array(props.position),
		rotation: new Float32Array(props.quaternion),
		scale: new Float32Array(props.scale),
	};
}
//#endregion 🔖Helpers

//#region 🔖TopologyClasses
export class Topology {
	protected readonly childIdList: readonly string[];

	constructor(readonly entity: TopologicEntity) {
		this.childIdList = childIds(entity);
	}

	get id(): string {
		return this.entity.id;
	}

	get kind(): TopologicKind {
		return this.entity.kind;
	}

	get label(): string {
		return entityLabel(this.entity);
	}

	get style(): TopologicStyle | undefined {
		return cloneStyle(this.entity.style);
	}

	get transform(): TopologicTransform | undefined {
		return this.entity.transform;
	}

	childIds(): readonly string[] {
		return this.childIdList;
	}

	children(model: SpatialModel): readonly Topology[] {
		return this.childIdList.map((id) => model.require(id));
	}

	toShape(model: SpatialModel): BrepShape | null {
		return model.shapeCache.get(this.id) ?? model.buildShape(this);
	}

	toRenderable(model: SpatialModel): SpatialRenderable {
		return model.renderableCache.get(this.id) ?? model.buildRenderable(this);
	}
}

export class Vertex extends Topology {
	declare readonly entity: TopologicVertexEntity;
}

export class Edge extends Topology {
	declare readonly entity: TopologicEdgeEntity;
}

export class Wire extends Topology {
	declare readonly entity: TopologicWireEntity;
}

export class Face extends Topology {
	declare readonly entity: TopologicFaceEntity;
}

export class Shell extends Topology {
	declare readonly entity: TopologicShellEntity;
}

export class Cell extends Topology {
	declare readonly entity: TopologicCellEntity;
}

export class CellComplex extends Topology {
	declare readonly entity: TopologicCellComplexEntity;
}

export class Cluster extends Topology {
	declare readonly entity: TopologicClusterEntity;
}

function childIds(entity: TopologicEntity): readonly string[] {
	if (entity.kind === "topology") return entity.members;
	if (entity.kind === "wire") return entity.edges;
	if (entity.kind === "face") return entity.wires;
	if (entity.kind === "shell") return entity.faces;
	if (entity.kind === "cell") return entity.shells;
	if (entity.kind === "cellComplex") return entity.cells;
	if (entity.kind === "cluster") return entity.topologies;
	return [];
	}
//#endregion 🔖TopologyClasses

//#region 🔖Model
function instantiate(entity: TopologicEntity): Topology {
	if (entity.kind === "vertex") return new Vertex(entity);
	if (entity.kind === "edge") return new Edge(entity);
	if (entity.kind === "wire") return new Wire(entity);
	if (entity.kind === "face") return new Face(entity);
	if (entity.kind === "shell") return new Shell(entity);
	if (entity.kind === "cell") return new Cell(entity);
	if (entity.kind === "cellComplex") return new CellComplex(entity);
	if (entity.kind === "cluster") return new Cluster(entity);
	return new Topology(entity);
	}

/** @emoji 🗺️ Imperative scene graph over the Topologic fixture with lazy brepjs shape creation. */
export class SpatialModel {
	readonly nodes: readonly Topology[];
	readonly nodeById: ReadonlyMap<string, Topology>;
	readonly shapeCache = new Map<string, BrepShape | null>();
	readonly renderableCache = new Map<string, SpatialRenderable>();

	constructor(readonly fixture: TopologicFixtureV1) {
		this.nodes = fixture.topologies.map(instantiate);
		this.nodeById = new Map(this.nodes.map((node) => [node.id, node]));
	}

	require(id: string): Topology {
		const entity = this.nodeById.get(id);
		if (!entity) throw new Error(`Unknown topology id: ${id}`);
		return entity;
	}

	get(id: string): Topology | undefined {
		return this.nodeById.get(id);
	}

	listByKind(kind: TopologicKind): readonly Topology[] {
		return this.nodes.filter((node) => node.kind === kind);
	}

	rootNodes(): readonly Topology[] {
		return this.fixture.roots.map((id) => this.require(id));
	}

	private vertexPoint(id: string): Vec3 {
		const vertex = this.require(id);
		if (!(vertex instanceof Vertex)) throw new Error(`Expected vertex for ${id}`);
		return vertex.entity.point;
	}

	private edgePoints(edge: Edge): readonly Vec3[] {
		if (edge.entity.curve && edge.entity.curve.length >= 2) return edge.entity.curve;
		return [this.vertexPoint(edge.entity.vertices[0]), this.vertexPoint(edge.entity.vertices[1])];
	}

	buildShape(node: Topology): BrepShape | null {
		const cached = this.shapeCache.get(node.id);
		if (cached !== undefined) return cached;
		let next: BrepShape | null = null;
		try {
			if (node instanceof Vertex) {
				next = null;
			} else if (node instanceof Edge) {
				const points = this.edgePoints(node);
				next = points.length > 2 ? unwrap(bsplineApprox(points)) : line(points[0], points[1]);
			} else if (node instanceof Wire) {
				const edges = node.entity.edges.map((id) => this.require(id).toShape(this)).filter(Boolean) as BrepShape[];
				next = node.entity.closed ? unwrap(wireLoop(edges)) : unwrap(wire(edges));
			} else if (node instanceof Face) {
				const wires = node.entity.wires.map((id) => this.require(id).toShape(this)).filter(Boolean) as BrepShape[];
				if (wires.length > 0) next = unwrap(brepFace(wires[0] as never, wires.slice(1) as never));
				else next = unwrap(polygon(node.entity.surface.vertices));
			} else if (node instanceof Shell) {
				const faces = node.entity.faces.map((id) => this.require(id).toShape(this)).filter(Boolean) as BrepShape[];
				next = unwrap(sewShells(faces as never));
			} else if (node instanceof Cell) {
				const shells = node.entity.shells.map((id) => this.require(id).toShape(this)).filter(Boolean) as BrepShape[];
				next = unwrap(solid(shells as never));
			} else if (node instanceof CellComplex) {
				const cells = node.entity.cells.map((id) => this.require(id).toShape(this)).filter(Boolean) as BrepShape[];
				next = compound(cells as never);
			} else if (node instanceof Cluster) {
				const topologies = node.entity.topologies.map((id) => this.require(id).toShape(this)).filter(Boolean) as BrepShape[];
				next = compound(topologies as never);
			} else if (node.entity.kind === "topology") {
				const members = node.entity.members.map((id) => this.require(id).toShape(this)).filter(Boolean) as BrepShape[];
				next = compound(members as never);
			}
		} catch {
			next = null;
		}
		this.shapeCache.set(node.id, next);
		return next;
	}

	buildRenderable(node: Topology): SpatialRenderable {
		const cached = this.renderableCache.get(node.id);
		if (cached) return cached;
		const shape = node.toShape(this);
		const fill = shape ? shapeFill(shape) : undefined;
		const edges = shape ? shapeLines(shape) : undefined;
		let renderable: SpatialRenderable;
		if (node instanceof Vertex) {
			renderable = {
				id: node.id,
				kind: node.kind,
				label: node.label,
				style: node.style,
				transform: node.transform,
				point: { position: node.entity.point, radius: node.entity.radius ?? node.style?.pointSize ?? 0.12 },
			};
		} else if (node instanceof Edge) {
			renderable = {
				id: node.id,
				kind: node.kind,
				label: node.label,
				style: node.style,
				transform: node.transform,
				edges: edges ?? { position: flattenPolyline(this.edgePoints(node), false) },
			};
		} else if (node instanceof Wire) {
			const points = node.entity.edges.flatMap((id) => {
				const edge = this.require(id);
				return edge instanceof Edge ? this.edgePoints(edge) : [];
			});
			renderable = {
				id: node.id,
				kind: node.kind,
				label: node.label,
				style: node.style,
				transform: node.transform,
				edges: edges ?? { position: flattenPolyline(points, Boolean(node.entity.closed)) },
			};
		} else if (node instanceof Face) {
			renderable = {
				id: node.id,
				kind: node.kind,
				label: node.label,
				style: node.style,
				transform: node.transform,
				fill: fill ?? faceSurfaceFill(node.entity),
				edges:
					edges ??
					{
						position: new Float32Array(
							node.entity.wires
								.flatMap((id) => this.require(id).toRenderable(this).edges?.position ? [...this.require(id).toRenderable(this).edges!.position] : []),
						),
					},
			};
		} else {
			const hasChildren = node.childIds().length > 0;
			const childRenderables = hasChildren ? node.children(this).map((child) => child.toRenderable(this)) : undefined;
			renderable = {
				id: node.id,
				kind: node.kind,
				label: node.label,
				style: node.style,
				transform: node.transform,
				fill: hasChildren ? undefined : fill,
				edges: hasChildren ? undefined : edges,
				children: childRenderables,
			};
		}
		this.renderableCache.set(node.id, renderable);
		return renderable;
	}
}

/** @emoji 🏗️ Builds the imperative spatial model from a parsed fixture. */
export function buildSpatialModel(fixture: TopologicFixtureV1): SpatialModel {
	return new SpatialModel(fixture);
}

/** @emoji 📚 Lists renderables for one topologic kind. */
export function listRenderablesByKind(model: SpatialModel, kind: TopologicKind): readonly SpatialRenderable[] {
	return model.listByKind(kind).map((node) => node.toRenderable(model));
}

export function buildTopologicRenderPacketV1(fixture: TopologicFixtureV1): TopologicRenderPacketV1 | null {
	if (!hasResolvableFixtureReferences(fixture)) return null;
	const model = buildSpatialModel(deriveAnalyzeTopologicFixtureV1(fixture) ?? fixture);
	return {
		entries: model.nodes.map((node) => {
			const transform = topologicTransformPacket(node.transform);
			if (node instanceof Edge) {
				return {
					id: node.id,
					kind: node.kind,
					...transform,
					points: new Float32Array(edgeCurveTopologicFixtureV1(model.fixture, node.id).flat()),
				};
			}
			if (node instanceof Face) {
				return {
					id: node.id,
					kind: node.kind,
					...transform,
					points: new Float32Array(node.entity.surface.vertices.flat()),
					triangles: new Uint32Array(node.entity.surface.triangles),
				};
			}
			return {
				id: node.id,
				kind: node.kind,
				...transform,
			};
		}),
		revisitedIds: [],
	};
}

/** @emoji 🏷️ Formats one spatial kind for panel labels and tool text. */
export function spatialKindLabel(kind: SpatialSurfaceKindFilter): string {
	if (kind === "all") return "All";
	if (kind === "cellComplex") return "CellComplex";
	return kind.charAt(0).toUpperCase() + kind.slice(1);
}

/** @emoji 🧾 Summarizes enabled spatial kind toggles for panel chrome. */
export function formatEnabledSpatialKindsLabel(kinds: Readonly<Record<TopologicKind, boolean>>): string {
	const enabled = TOPOLOGIC_KINDS.filter((kind) => kinds[kind]);
	return enabled.length === TOPOLOGIC_KINDS.length ? "all" : enabled.join(", ") || "none";
}

function matchesSpatialPanelQuery(node: Topology, query: string): boolean {
	const value = query.trim().toLowerCase();
	if (!value) return true;
	const haystack = [node.id, node.label, node.kind, node.entity.description]
		.filter((entry): entry is string => Boolean(entry))
		.join(" ")
		.toLowerCase();
	return haystack.includes(value);
}

/** @emoji 📚 Projects visible/filter-matching nodes for workbench browsing outside the renderer. */
export function listSpatialPanelNodes(args: {
	readonly model: SpatialModel | null;
	readonly focusedKind: SpatialSurfaceKindFilter;
	readonly visibleKinds: Readonly<Record<TopologicKind, boolean>>;
	readonly query: string;
}): readonly Topology[] {
	if (!args.model) return [];
	const nodes = args.focusedKind === "all" ? args.model.nodes : args.model.listByKind(args.focusedKind);
	return nodes.filter((node) => args.visibleKinds[node.kind] && matchesSpatialPanelQuery(node, args.query));
}

function filterSpatialRenderableForest(renderable: SpatialRenderable, visibleKinds: Readonly<Record<TopologicKind, boolean>>): readonly SpatialRenderable[] {
	const children = renderable.children?.flatMap((child) => filterSpatialRenderableForest(child, visibleKinds)) ?? [];
	if (!visibleKinds[renderable.kind]) return children;
	return [{ ...renderable, children: children.length > 0 ? children : undefined }];
}

/** @emoji 🪟 Projects viewport renderables from class-side visibility and focus state. */
export function listSpatialViewportRenderables(args: {
	readonly model: SpatialModel | null;
	readonly focusedKind: SpatialSurfaceKindFilter;
	readonly visibleKinds: Readonly<Record<TopologicKind, boolean>>;
}): readonly SpatialRenderable[] {
	if (!args.model) return [];
	if (args.focusedKind === "all") {
		return args.model.rootNodes().flatMap((node) => filterSpatialRenderableForest(node.toRenderable(args.model!), args.visibleKinds));
	}
	if (!args.visibleKinds[args.focusedKind]) return [];
	return listRenderablesByKind(args.model, args.focusedKind);
}

function selectedSpatialNode(model: SpatialModel | null, selectedId: string | null): Topology | null {
	return selectedId && model ? model.get(selectedId) ?? null : null;
}

/** @emoji 🧭 Builds renderer-agnostic state for the workbench browsing tab. */
export function buildSpatialWorkbenchPanelState(args: {
	readonly fixtureLabel?: string;
	readonly model: SpatialModel | null;
	readonly focusedKind: SpatialSurfaceKindFilter;
	readonly selectedId: string | null;
	readonly query: string;
	readonly selectableKinds: Readonly<Record<TopologicKind, boolean>>;
	readonly visibleKinds: Readonly<Record<TopologicKind, boolean>>;
	readonly setFocusedKind: (kind: SpatialSurfaceKindFilter) => void;
	readonly setSelectedId: (id: string | null) => void;
	readonly setQuery: (query: string) => void;
}): SpatialWorkbenchPanelState {
	const entities = listSpatialPanelNodes({ model: args.model, focusedKind: args.focusedKind, visibleKinds: args.visibleKinds, query: args.query });
	return {
		fixtureLabel: args.fixtureLabel,
		visibleKindsLabel: formatEnabledSpatialKindsLabel(args.visibleKinds),
		selectableKindsLabel: formatEnabledSpatialKindsLabel(args.selectableKinds),
		query: args.query,
		focusOptions: (["all", ...TOPOLOGIC_KINDS] as const).map((kind) => ({ kind, label: spatialKindLabel(kind), active: args.focusedKind === kind })),
		entityCount: entities.length,
		entities: entities.map((entity) => ({ id: entity.id, label: entity.label, kindLabel: entity.kind, selected: args.selectedId === entity.id })),
		setFocusedKind: args.setFocusedKind,
		setSelectedId: args.setSelectedId,
		setQuery: args.setQuery,
	};
}

/** @emoji 🔎 Builds renderer-agnostic state for the selection details tab. */
export function buildSpatialDetailsPanelState(args: {
	readonly status: SpatialStatus;
	readonly model: SpatialModel | null;
	readonly focusedKind: SpatialSurfaceKindFilter;
	readonly selectedId: string | null;
	readonly query: string;
}): SpatialDetailsPanelState {
	const selected = selectedSpatialNode(args.model, args.selectedId);
	return {
		selectedLabel: selected ? selected.label : "No entity selected",
		selectedKindLabel: selected?.kind ?? spatialKindLabel(args.focusedKind),
		description: selected?.entity.description,
		status: args.status,
		focusedKindLabel: spatialKindLabel(args.focusedKind),
		query: args.query.trim() || "none",
	};
}

/** @emoji 📐 Converts a transform into R3F-friendly tuples. */
export function transformProps(transform: TopologicTransform | undefined): {
	readonly position: Vec3;
	readonly quaternion: Quat;
	readonly scale: Vec3;
} {
	return {
		position: transform?.position ?? [0, 0, 0],
		quaternion: transform?.rotation ?? [0, 0, 0, 1],
		scale: transformScale(transform),
	};
}
//#endregion 🔖Model

export {
	ensureSpatialJsBindingsLoaded as ensureTopologicJsBindingsLoaded,
	getSpatialJsBindings as getTopologicJsBindings,
	type SpatialJsBindings as TopologicJsBindings,
};

export async function ensureTopologicWasmLoaded(): Promise<TopologicWasmBindings> {
	return ensureSpatialJsBindingsLoaded();
}

if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;
	const topologyJson = (await import("../../play/fixtures/topology.json")).default;

	describe("spatial imperative core", () => {
		it("rejects unresolved references like the topologic facade", () => {
			expect(
				parseTopologicFixtureV1({
					schema: "elements.geometry.topologic.fixture/v1",
					roots: ["root"],
					topologies: [{ id: "root", kind: "topology", members: ["missing"] }],
				}),
			).toBeNull();
		});

		it(
			"parses the shared topologic fixture and builds brep-backed renderables",
			async () => {
				await ensureSpatialKernelLoaded();
				const fixture = parseTopologicFixtureV1(topologyJson);
				expect(fixture).not.toBeNull();
				const model = buildSpatialModel(fixture!);
				expect(model.listByKind("cell").length).toBeGreaterThan(0);
				const cell = model.listByKind("cell")[0];
				expect(cell).toBeInstanceOf(Cell);
				const renderable = cell.toRenderable(model);
				expect((renderable.fill?.position.length ?? 0) > 0 || (renderable.children?.length ?? 0) > 0).toBe(true);
				const rootRenderable = model.rootNodes()[0]?.toRenderable(model);
				expect(rootRenderable?.children?.length ?? 0).toBeGreaterThan(0);
			},
			60000,
		);

		it("updates one entity transform immutably", () => {
			const fixture = parseTopologicFixtureV1(topologyJson)!;
			const next = updateTopologicFixtureTransformV1(fixture, "cell-room", { position: [1, 2, 3] });
			const updated = next?.topologies.find((entity) => entity.id === "cell-room");
			expect(updated?.transform?.position).toEqual([1, 2, 3]);
			expect(fixture.topologies.find((entity) => entity.id === "cell-room")?.transform).toBeUndefined();
		});

		it("exports topologic-compatible bindings and render packets", async () => {
			const fixture = await loadTopologicFixtureV1(topologyJson);
			expect(fixture).not.toBeNull();
			const bindings = await ensureTopologicWasmLoaded();
			expect(bindings.parseFixture(topologyJson)?.schema).toBe("elements.geometry.topologic.fixture/v1");
			expect(bindings.edgeCurve(fixture!, "edge-floor-front").length).toBeGreaterThanOrEqual(2);
			const packet = buildTopologicRenderPacketV1(fixture!);
			expect(packet?.entries.find((entry) => entry.id === "edge-floor-front")?.points).toBeInstanceOf(Float32Array);
		});
	});
}
