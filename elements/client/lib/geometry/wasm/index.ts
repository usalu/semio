// #region 🧲Header
// 💻 elements/client/lib/geometry/wasm/index.ts — Topologic wasm adapter: keeps the typed TS contract while delegating fixture validation and geometry queries into the compiled C++/embind module.
// #endregion 🧲Header

import { ensureTopologicJsBindingsLoaded, getTopologicJsBindings } from "../js/index.ts";

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
function parseVec3(value: unknown): Vec3 | null {
	if (!Array.isArray(value) || value.length !== 3) return null;
	return value.every((entry) => typeof entry === "number" && Number.isFinite(entry)) ? (value as Vec3) : null;
}

function parseQuat(value: unknown): Quat | null {
	if (!Array.isArray(value) || value.length !== 4) return null;
	return value.every((entry) => typeof entry === "number" && Number.isFinite(entry)) ? (value as Quat) : null;
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
	return (getTopologicJsBindings().parseFixture(raw) as TopologicFixtureV1 | null) ?? null;
	}

export async function loadTopologicFixtureV1(raw: unknown): Promise<TopologicFixtureV1 | null> {
	await ensureTopologicJsBindingsLoaded();
	return parseTopologicFixtureV1(raw);
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
		return (getTopologicJsBindings().vertexPoint(this.fixture, id) as Vec3 | null) ?? null;
	}

	edgeCurve(id: string): readonly Vec3[] {
		return (getTopologicJsBindings().edgeCurve(this.fixture, id) as readonly Vec3[]) ?? [];
	}
	}

export interface TopologicWasmBindings {
	readonly parseFixture: (raw: unknown) => TopologicFixtureV1 | null;
	readonly createSession: (fixture: TopologicFixtureV1) => TopologicWasmSession;
}

export async function ensureTopologicWasmLoaded(): Promise<TopologicWasmBindings> {
	await ensureTopologicJsBindingsLoaded();
	return {
		parseFixture: parseTopologicFixtureV1,
		createSession: (fixture) => new TopologicWasmSession(fixture),
	};
	}

export function updateTopologicFixtureTransform(
	fixture: TopologicFixtureV1,
	entityId: string,
	transform: TopologicTransform,
): TopologicFixtureV1 {
	return getTopologicJsBindings().updateFixtureTransform(fixture, entityId, transform) as TopologicFixtureV1;
	}
//#endregion 🔖Session

//#region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("parseTopologicFixtureV1", () => {
		it("accepts a valid fixture with every topologic kind", async () => {
			const fixture = await loadTopologicFixtureV1({
				schema: "elements.geometry.topologic.fixture/v1",
				roots: ["root"],
				topologies: [
					{ id: "root", kind: "topology", members: ["vertex"] },
					{ id: "vertex", kind: "vertex", point: [0, 0, 0] },
				],
			});
			expect(fixture?.roots).toEqual(["root"]);
		});

		it("rejects unresolved references", async () => {
			expect(
				await loadTopologicFixtureV1({
					schema: "elements.geometry.topologic.fixture/v1",
					roots: ["root"],
					topologies: [{ id: "root", kind: "topology", members: ["missing"] }],
				}),
			).toBeNull();
		});
	});

	describe("updateTopologicFixtureTransform", () => {
		it("updates only the targeted entity", async () => {
			const fixture = await loadTopologicFixtureV1({
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