// #region 🧲Header
// 💻 elements/client/lib/geometry/wasm/index.ts — Topologic browser binding facade: validates fixture JSON, exposes session-style queries, and applies editable transforms for the R3F layer.
// #endregion 🧲Header

//#region 🔖Kinds
export const TOPOLOGIC_KINDS = [
	"topology",
	"vertex",
	"edge",
	"wire",
	"face",
	"shell",
	"cell",
	"cellComplex",
	"cluster",
] as const;

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
//#endregion 🔖Kinds

//#region 🔖Parsing
function isRecord(value: unknown): value is Record<string, unknown> {
	return Boolean(value) && typeof value === "object" && !Array.isArray(value);
}

function isColor(value: unknown): value is string {
	return typeof value === "string" && value.trim().length > 0;
}

function parseVec3(value: unknown): Vec3 | null {
	if (!Array.isArray(value) || value.length !== 3) return null;
	const out = value.map((entry) => (typeof entry === "number" && Number.isFinite(entry) ? entry : Number.NaN));
	return out.every((entry) => Number.isFinite(entry)) ? (out as Vec3) : null;
}

function parseQuat(value: unknown): Quat | null {
	if (!Array.isArray(value) || value.length !== 4) return null;
	const out = value.map((entry) => (typeof entry === "number" && Number.isFinite(entry) ? entry : Number.NaN));
	return out.every((entry) => Number.isFinite(entry)) ? (out as Quat) : null;
}

function parseStyle(value: unknown): TopologicStyle | undefined {
	if (!isRecord(value)) return undefined;
	const style: TopologicStyle = {};
	if (isColor(value.color)) style.color = value.color;
	if (isColor(value.edgeColor)) style.edgeColor = value.edgeColor;
	if (typeof value.opacity === "number" && Number.isFinite(value.opacity)) style.opacity = value.opacity;
	if (typeof value.lineWidth === "number" && Number.isFinite(value.lineWidth)) style.lineWidth = value.lineWidth;
	if (typeof value.pointSize === "number" && Number.isFinite(value.pointSize)) style.pointSize = value.pointSize;
	return Object.keys(style).length > 0 ? style : undefined;
}

function parseTransform(value: unknown): TopologicTransform | undefined {
	if (!isRecord(value)) return undefined;
	const transform: TopologicTransform = {};
	const position = parseVec3(value.position);
	const rotation = parseQuat(value.rotation);
	if (position) transform.position = position;
	if (rotation) transform.rotation = rotation;
	if (typeof value.scale === "number" && Number.isFinite(value.scale)) transform.scale = value.scale;
	else {
		const scale = parseVec3(value.scale);
		if (scale) transform.scale = scale;
	}
	return Object.keys(transform).length > 0 ? transform : undefined;
}

function parseBase(value: Record<string, unknown>): Omit<TopologicEntityBase, "kind"> | null {
	if (typeof value.id !== "string" || value.id.trim().length === 0) return null;
	const base: Omit<TopologicEntityBase, "kind"> = { id: value.id };
	if (typeof value.label === "string" && value.label.trim().length > 0) base.label = value.label;
	if (typeof value.description === "string" && value.description.trim().length > 0) base.description = value.description;
	const style = parseStyle(value.style);
	const transform = parseTransform(value.transform);
	if (style) base.style = style;
	if (transform) base.transform = transform;
	if (isRecord(value.metadata)) base.metadata = value.metadata;
	return base;
}

function parseStringArray(value: unknown): string[] | null {
	if (!Array.isArray(value) || value.some((entry) => typeof entry !== "string" || entry.trim().length === 0)) return null;
	return value as string[];
}

function parseSurface(value: unknown): TopologicFaceEntity["surface"] | null {
	if (!isRecord(value)) return null;
	if (!Array.isArray(value.vertices) || !Array.isArray(value.triangles)) return null;
	const vertices = value.vertices.map((entry) => parseVec3(entry));
	if (vertices.some((entry) => !entry)) return null;
	const triangles = value.triangles.map((entry) => (typeof entry === "number" && Number.isInteger(entry) ? entry : Number.NaN));
	if (triangles.some((entry) => !Number.isInteger(entry))) return null;
	return {
		vertices: vertices as Vec3[],
		triangles,
	};
	}

function parseEntity(raw: unknown): TopologicEntity | null {
	if (!isRecord(raw) || typeof raw.kind !== "string" || !TOPOLOGIC_KINDS.includes(raw.kind as TopologicKind)) return null;
	const base = parseBase(raw);
	if (!base) return null;
	if (raw.kind === "topology") {
		const members = parseStringArray(raw.members);
		return members ? { ...base, kind: "topology", members } : null;
	}
	if (raw.kind === "vertex") {
		const point = parseVec3(raw.point);
		return point
			? {
				...base,
				kind: "vertex",
				point,
				...(typeof raw.radius === "number" && Number.isFinite(raw.radius) ? { radius: raw.radius } : {}),
			}
			: null;
	}
	if (raw.kind === "edge") {
		if (!Array.isArray(raw.vertices) || raw.vertices.length !== 2 || raw.vertices.some((entry) => typeof entry !== "string")) return null;
		const curve = Array.isArray(raw.curve) ? raw.curve.map((entry) => parseVec3(entry)) : undefined;
		if (curve && curve.some((entry) => !entry)) return null;
		return {
			...base,
			kind: "edge",
			vertices: [raw.vertices[0] as string, raw.vertices[1] as string],
			...(curve ? { curve: curve as Vec3[] } : {}),
		};
	}
	if (raw.kind === "wire") {
		const edges = parseStringArray(raw.edges);
		return edges
			? {
				...base,
				kind: "wire",
				edges,
				...(raw.closed === true ? { closed: true } : {}),
				...(raw.manifold === false ? { manifold: false } : { manifold: true }),
			}
			: null;
	}
	if (raw.kind === "face") {
		const wires = parseStringArray(raw.wires);
		const surface = parseSurface(raw.surface);
		return wires && surface ? { ...base, kind: "face", wires, surface } : null;
	}
	if (raw.kind === "shell") {
		const faces = parseStringArray(raw.faces);
		return faces ? { ...base, kind: "shell", faces } : null;
	}
	if (raw.kind === "cell") {
		const shells = parseStringArray(raw.shells);
		return shells ? { ...base, kind: "cell", shells } : null;
	}
	if (raw.kind === "cellComplex") {
		const cells = parseStringArray(raw.cells);
		return cells ? { ...base, kind: "cellComplex", cells } : null;
	}
	const topologies = parseStringArray(raw.topologies);
	return topologies ? { ...base, kind: "cluster", topologies } : null;
	}

function childrenOf(entity: TopologicEntity): readonly string[] {
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

export function parseTopologicFixtureV1(raw: unknown): TopologicFixtureV1 | null {
	if (!isRecord(raw) || raw.schema !== "elements.geometry.topologic.fixture/v1") return null;
	const roots = parseStringArray(raw.roots);
	if (!roots || !Array.isArray(raw.topologies)) return null;
	const topologies = raw.topologies.map((entry) => parseEntity(entry));
	if (topologies.some((entry) => !entry)) return null;
	const entityById = new Map<string, TopologicEntity>();
	for (const entity of topologies as TopologicEntity[]) {
		if (entityById.has(entity.id)) return null;
		entityById.set(entity.id, entity);
	}
	for (const id of roots) {
		if (!entityById.has(id)) return null;
	}
	for (const entity of entityById.values()) {
		for (const childId of childrenOf(entity)) {
			if (!entityById.has(childId)) return null;
		}
		if (entity.kind === "face") {
			const maxIndex = entity.surface.vertices.length - 1;
			if (entity.surface.triangles.length % 3 !== 0) return null;
			if (entity.surface.triangles.some((index) => index < 0 || index > maxIndex)) return null;
		}
	}
	return {
		schema: "elements.geometry.topologic.fixture/v1",
		...(typeof raw.label === "string" ? { label: raw.label } : {}),
		roots,
		topologies: topologies as TopologicEntity[],
	};
	}
//#endregion 🔖Parsing

//#region 🔖Math
export function normalizeTransform(transform: TopologicTransform | undefined): Required<TopologicTransform> {
	return {
		position: transform?.position ?? [0, 0, 0],
		rotation: transform?.rotation ?? [0, 0, 0, 1],
		scale: transform?.scale ?? 1,
	};
	}

function normalizeScale(scale: number | Vec3): Vec3 {
	return typeof scale === "number" ? [scale, scale, scale] : scale;
	}

export function topologicTransformPoint(point: Vec3, transform: TopologicTransform | undefined): Vec3 {
	const normalized = normalizeTransform(transform);
	const [sx, sy, sz] = normalizeScale(normalized.scale);
	const [px, py, pz] = normalized.position;
	const [qx, qy, qz, qw] = normalized.rotation;
	const x = point[0] * sx;
	const y = point[1] * sy;
	const z = point[2] * sz;
	const ix = qw * x + qy * z - qz * y;
	const iy = qw * y + qz * x - qx * z;
	const iz = qw * z + qx * y - qy * x;
	const iw = -qx * x - qy * y - qz * z;
	return [
		ix * qw + iw * -qx + iy * -qz - iz * -qy + px,
		iy * qw + iw * -qy + iz * -qx - ix * -qz + py,
		iz * qw + iw * -qz + ix * -qy - iy * -qx + pz,
	];
	}

export function topologicEntityLabel(entity: TopologicEntity): string {
	return entity.label ?? entity.id;
	}
//#endregion 🔖Math

//#region 🔖Session
export class TopologicWasmSession {
	readonly entityById: ReadonlyMap<string, TopologicEntity>;

	constructor(readonly fixture: TopologicFixtureV1) {
		this.entityById = new Map(fixture.topologies.map((entity) => [entity.id, entity]));
	}

	getEntity(id: string): TopologicEntity | undefined {
		return this.entityById.get(id);
	}

	listByKind(kind: TopologicKind): readonly TopologicEntity[] {
		return this.fixture.topologies.filter((entity) => entity.kind === kind);
	}

	childrenOf(id: string): readonly TopologicEntity[] {
		const entity = this.entityById.get(id);
		if (!entity) return [];
		return childrenOf(entity)
			.map((childId) => this.entityById.get(childId))
			.filter((child): child is TopologicEntity => Boolean(child));
	}

	vertexPoint(id: string): Vec3 | null {
		const entity = this.entityById.get(id);
		return entity?.kind === "vertex" ? topologicTransformPoint(entity.point, entity.transform) : null;
	}

	edgeCurve(id: string): readonly Vec3[] {
		const entity = this.entityById.get(id);
		if (!entity || entity.kind !== "edge") return [];
		if (entity.curve && entity.curve.length >= 2) return entity.curve;
		const start = this.vertexPoint(entity.vertices[0]);
		const end = this.vertexPoint(entity.vertices[1]);
		return start && end ? [start, end] : [];
	}
	}

export interface TopologicWasmBindings {
	readonly parseFixture: (raw: unknown) => TopologicFixtureV1 | null;
	readonly createSession: (fixture: TopologicFixtureV1) => TopologicWasmSession;
}

const TOPLOGIC_BROWSER_BINDINGS: TopologicWasmBindings = {
	parseFixture: parseTopologicFixtureV1,
	createSession: (fixture) => new TopologicWasmSession(fixture),
};

/** @emoji 🌐 Browser-facing Topologic bindings contract for geometry play; stays async so a future native wasm payload can slot into the same surface. */
export async function ensureTopologicWasmLoaded(): Promise<TopologicWasmBindings> {
	return TOPLOGIC_BROWSER_BINDINGS;
	}

export function updateTopologicFixtureTransform(
	fixture: TopologicFixtureV1,
	entityId: string,
	transform: TopologicTransform,
): TopologicFixtureV1 {
	return {
		...fixture,
		topologies: fixture.topologies.map((entity) => (entity.id === entityId ? { ...entity, transform } : entity)),
	};
	}
//#endregion 🔖Session

//#region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("parseTopologicFixtureV1", () => {
		it("accepts a valid fixture with every topologic kind", () => {
			const fixture = parseTopologicFixtureV1({
				schema: "elements.geometry.topologic.fixture/v1",
				roots: ["root"],
				topologies: [
					{ id: "root", kind: "topology", members: ["vertex"] },
					{ id: "vertex", kind: "vertex", point: [0, 0, 0] },
				],
			});
			expect(fixture?.roots).toEqual(["root"]);
		});

		it("rejects unresolved references", () => {
			expect(
				parseTopologicFixtureV1({
					schema: "elements.geometry.topologic.fixture/v1",
					roots: ["root"],
					topologies: [{ id: "root", kind: "topology", members: ["missing"] }],
				}),
			).toBeNull();
		});
	});

	describe("updateTopologicFixtureTransform", () => {
		it("updates only the targeted entity", () => {
			const fixture = parseTopologicFixtureV1({
				schema: "elements.geometry.topologic.fixture/v1",
				roots: ["root"],
				topologies: [
					{ id: "root", kind: "topology", members: ["vertex"] },
					{ id: "vertex", kind: "vertex", point: [0, 0, 0] },
				],
			}) as TopologicFixtureV1;
			const updated = updateTopologicFixtureTransform(fixture, "vertex", { position: [3, 2, 1] });
			const session = new TopologicWasmSession(updated);
			expect(session.getEntity("vertex")).toMatchObject({ transform: { position: [3, 2, 1] } });
		});
	});
}
//#endregion 🧪Tests