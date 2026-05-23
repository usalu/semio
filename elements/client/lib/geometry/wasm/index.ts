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

function normalizeScaleVector(scale: TopologicTransform["scale"]): Vec3 {
	return normalizeScale(scale ?? 1);
	}

function multiplyQuat(
	a: Quat,
	b: Quat,
): Quat {
	const [ax, ay, az, aw] = a;
	const [bx, by, bz, bw] = b;
	return [
		aw * bx + ax * bw + ay * bz - az * by,
		aw * by - ax * bz + ay * bw + az * bx,
		aw * bz + ax * by - ay * bx + az * bw,
		aw * bw - ax * bx - ay * by - az * bz,
	];
	}

export function composeTransforms(
	parent: TopologicTransform | undefined,
	local: TopologicTransform | undefined,
): Required<TopologicTransform> {
	if (!parent) return normalizeTransform(local);
	if (!local) return normalizeTransform(parent);
	const parentNormalized = normalizeTransform(parent);
	const localNormalized = normalizeTransform(local);
	const parentScale = normalizeScaleVector(parentNormalized.scale);
	const localScale = normalizeScaleVector(localNormalized.scale);
	return {
		position: topologicTransformPoint(localNormalized.position, parentNormalized),
		rotation: multiplyQuat(parentNormalized.rotation, localNormalized.rotation),
		scale: [parentScale[0] * localScale[0], parentScale[1] * localScale[1], parentScale[2] * localScale[2]],
	};
	}

export function inverseTransform(transform: TopologicTransform | undefined): Required<TopologicTransform> {
	const normalized = normalizeTransform(transform);
	const [sx, sy, sz] = normalizeScaleVector(normalized.scale);
	const [qx, qy, qz, qw] = normalized.rotation;
	const invRotation: Quat = [-qx, -qy, -qz, qw];
	const invScale: Vec3 = [1 / sx, 1 / sy, 1 / sz];
	const [px, py, pz] = normalized.position;
	return {
		position: topologicTransformPoint([-px, -py, -pz], { rotation: invRotation, scale: invScale }),
		rotation: invRotation,
		scale: invScale,
	};
	}

export function centroid(points: readonly Vec3[]): Vec3 {
	if (points.length === 0) return [0, 0, 0];
	const sum = points.reduce<[number, number, number]>(
		(accumulator, point) => [accumulator[0] + point[0], accumulator[1] + point[1], accumulator[2] + point[2]],
		[0, 0, 0],
	);
	return [sum[0] / points.length, sum[1] / points.length, sum[2] / points.length];
	}

export function collectDescendantIds(session: TopologicWasmSession, entityId: string): readonly string[] {
	const descendants: string[] = [];
	const visit = (id: string): void => {
		for (const child of session.childrenOf(id)) {
			descendants.push(child.id);
			visit(child.id);
		}
	};
	visit(entityId);
	return descendants;
	}

function findEntityPathFromRoots(session: TopologicWasmSession, entityId: string): readonly string[] | null {
	const visit = (id: string, path: readonly string[]): readonly string[] | null => {
		const nextPath = [...path, id];
		if (id === entityId) return nextPath;
		for (const child of session.childrenOf(id)) {
			const found = visit(child.id, nextPath);
			if (found) return found;
		}
		return null;
	};
	for (const rootId of session.fixture.roots) {
		const found = visit(rootId, []);
		if (found) return found;
	}
	return null;
	}

export function resolveEntityRenderTransform(
	session: TopologicWasmSession,
	entity: TopologicEntity,
	inherited: TopologicTransform | undefined,
): TopologicTransform | undefined {
	if (entity.kind === "vertex") {
		return composeTransforms(composeTransforms(inherited, entity.transform), { position: entity.point });
	}
	if (entity.kind === "edge") {
		const anchor = centroid(session.edgeCurve(entity.id));
		return composeTransforms(composeTransforms(inherited, entity.transform), { position: anchor });
	}
	if (entity.kind === "wire") {
		const points = entity.edges.flatMap((edgeId) => [...session.edgeCurve(edgeId)]);
		const anchor = centroid(points);
		return composeTransforms(composeTransforms(inherited, entity.transform), { position: anchor });
	}
	if (entity.kind === "face") {
		const anchor = centroid(entity.surface.vertices);
		return composeTransforms(composeTransforms(inherited, entity.transform), { position: anchor });
	}
	return composeTransforms(inherited, entity.transform);
	}

function applyTransformDeltaToEntity(entity: TopologicEntity, delta: TopologicTransform): TopologicEntity {
	const { transform: _transform, ...rest } = entity;
	if (entity.kind === "vertex") {
		return { ...rest, kind: "vertex", point: topologicTransformPoint(entity.point, delta) };
	}
	if (entity.kind === "edge") {
		return {
			...rest,
			kind: "edge",
			vertices: entity.vertices,
			...(entity.curve
				? { curve: entity.curve.map((point) => topologicTransformPoint(point, delta)) as readonly Vec3[] }
				: {}),
		};
	}
	if (entity.kind === "face") {
		return {
			...rest,
			kind: "face",
			wires: entity.wires,
			surface: {
				vertices: entity.surface.vertices.map((point) => topologicTransformPoint(point, delta)),
				triangles: entity.surface.triangles,
			},
		};
	}
	return rest as TopologicEntity;
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
	const session = new TopologicWasmSession(fixture);
	const entity = session.getEntity(entityId);
	if (!entity) return fixture;
	const path = findEntityPathFromRoots(session, entityId);
	if (!path) return fixture;
	let inherited: TopologicTransform | undefined;
	for (const pathId of path) {
		const pathEntity = session.getEntity(pathId);
		if (!pathEntity) return fixture;
		if (pathId === entityId) break;
		inherited = resolveEntityRenderTransform(session, pathEntity, inherited);
	}
	const previousWorld = resolveEntityRenderTransform(session, entity, inherited);
	const nextWorld = normalizeTransform(transform);
	const delta = composeTransforms(nextWorld, inverseTransform(previousWorld));
	const affectedIds = new Set<string>([entityId, ...collectDescendantIds(session, entityId)]);
	return {
		...fixture,
		topologies: fixture.topologies.map((topology) =>
			affectedIds.has(topology.id) ? applyTransformDeltaToEntity(topology, delta) : topology,
		),
	};
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
		it("bakes a vertex drag into point geometry", async () => {
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
			expect(session.getEntity("vertex")).toMatchObject({ point: [3, 2, 1] });
			expect(session.getEntity("vertex")?.transform).toBeUndefined();
		});

		it("moves face descendants when a face is translated", async () => {
			const fixture = (await loadTopologicFixtureV1({
				schema: "elements.geometry.topologic.fixture/v1",
				roots: ["root"],
				topologies: [
					{ id: "root", kind: "topology", members: ["face"] },
					{ id: "v0", kind: "vertex", point: [0, 0, 0] },
					{ id: "v1", kind: "vertex", point: [2, 0, 0] },
					{ id: "v2", kind: "vertex", point: [2, 0, 2] },
					{ id: "e0", kind: "edge", vertices: ["v0", "v1"] },
					{ id: "e1", kind: "edge", vertices: ["v1", "v2"] },
					{ id: "wire", kind: "wire", edges: ["e0", "e1"] },
					{
						id: "face",
						kind: "face",
						wires: ["wire"],
						surface: {
							vertices: [
								[0, 0, 0],
								[2, 0, 0],
								[2, 0, 2],
							],
							triangles: [0, 1, 2],
						},
					},
				],
			})) as TopologicFixtureV1;
			const session = new TopologicWasmSession(fixture);
			const faceWorld = resolveEntityRenderTransform(session, session.getEntity("face") as TopologicFaceEntity, undefined);
			const updated = updateTopologicFixtureTransform(fixture, "face", {
				position: [faceWorld?.position?.[0] ?? 0, (faceWorld?.position?.[1] ?? 0) + 5, faceWorld?.position?.[2] ?? 0],
				rotation: faceWorld?.rotation ?? [0, 0, 0, 1],
				scale: faceWorld?.scale ?? 1,
			});
			const nextSession = new TopologicWasmSession(updated);
			expect(nextSession.getEntity("v0")).toMatchObject({ point: [0, 5, 0] });
			expect(nextSession.getEntity("v1")).toMatchObject({ point: [2, 5, 0] });
			expect(nextSession.getEntity("v2")).toMatchObject({ point: [2, 5, 2] });
			expect(nextSession.vertexPoint("v0")).toEqual([0, 5, 0]);
		});

		it("computes edge curves when vertices omit transforms", async () => {
			const fixture = (await loadTopologicFixtureV1({
				schema: "elements.geometry.topologic.fixture/v1",
				roots: ["root"],
				topologies: [
					{ id: "root", kind: "topology", members: ["edge"] },
					{ id: "start", kind: "vertex", point: [0, 0, 0] },
					{ id: "end", kind: "vertex", point: [2, 0, 0] },
					{ id: "edge", kind: "edge", vertices: ["start", "end"] },
				],
			})) as TopologicFixtureV1;
			const session = new TopologicWasmSession(fixture);
			expect(session.edgeCurve("edge")).toEqual([
				[0, 0, 0],
				[2, 0, 0],
			]);
		});
	});
}
//#endregion 🧪Tests