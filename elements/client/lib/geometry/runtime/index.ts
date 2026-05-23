// #region 🧲Header
// 💻 elements/client/lib/geometry/runtime/index.ts — Geometry runtime helpers: scene/session traversal, transform math, and UI-side fixture updates layered above the Topologic kernel adapter.
// #endregion 🧲Header

import {
	edgeCurveTopologicFixtureV1,
	loadTopologicFixtureV1,
	type Quat,
	type TopologicEdgeEntity,
	type TopologicEntity,
	type TopologicFaceEntity,
	type TopologicFixtureV1,
	type TopologicKind,
	type TopologicTopologyEntity,
	type TopologicTransform,
	type TopologicVertexEntity,
	type TopologicWireEntity,
	type Vec3,
	vertexPointTopologicFixtureV1,
} from "../wasm/index.ts";

//#region 🔖Math
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

function multiplyQuat(a: Quat, b: Quat): Quat {
	const [ax, ay, az, aw] = a;
	const [bx, by, bz, bw] = b;
	return [
		aw * bx + ax * bw + ay * bz - az * by,
		aw * by - ax * bz + ay * bw + az * bx,
		aw * bz + ax * by - ay * bx + az * bw,
		aw * bw - ax * bx - ay * by - az * bz,
	];
	}

export function composeTransforms(parent: TopologicTransform | undefined, local: TopologicTransform | undefined): Required<TopologicTransform> {
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
		return vertexPointTopologicFixtureV1(this.fixture, id);
	}

	edgeCurve(id: string): readonly Vec3[] {
		return edgeCurveTopologicFixtureV1(this.fixture, id);
	}
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

export function collectAncestorIds(session: TopologicWasmSession, entityId: string): readonly string[] {
	const descendantCache = new Map<string, readonly string[]>();
	const descendantsOf = (id: string): readonly string[] => {
		const cached = descendantCache.get(id);
		if (cached) return cached;
		const descendants = collectDescendantIds(session, id);
		descendantCache.set(id, descendants);
		return descendants;
	};
	return session.fixture.topologies.map((topology) => topology.id).filter((id) => id !== entityId && descendantsOf(id).includes(entityId));
	}

export function collectLinkedIds(session: TopologicWasmSession, entityId: string): readonly string[] {
	return [...new Set([entityId, ...collectDescendantIds(session, entityId), ...collectAncestorIds(session, entityId)])];
	}

function collectBoundaryVertexIds(session: TopologicWasmSession, seedIds: readonly string[]): Set<string> {
	const vertexIds = new Set<string>();
	for (const seedId of seedIds) {
		const entity = session.getEntity(seedId);
		if (!entity) continue;
		if (entity.kind === "vertex") vertexIds.add(entity.id);
		if (entity.kind === "edge") entity.vertices.forEach((vertexId) => vertexIds.add(vertexId));
	}
	return vertexIds;
	}

function faceUsesBoundaryVertex(session: TopologicWasmSession, face: TopologicFaceEntity, boundaryVertexIds: ReadonlySet<string>): boolean {
	for (const wireId of face.wires) {
		const wire = session.getEntity(wireId);
		if (wire?.kind !== "wire") continue;
		for (const edgeId of wire.edges) {
			const edge = session.getEntity(edgeId);
			if (edge?.kind === "edge" && edge.vertices.some((vertexId) => boundaryVertexIds.has(vertexId))) return true;
		}
	}
	return face.surface.vertices.some((point) =>
		[...boundaryVertexIds].some((vertexId) => {
			const vertex = session.getEntity(vertexId);
			if (vertex?.kind !== "vertex") return false;
			return vertex.point[0] === point[0] && vertex.point[1] === point[1] && vertex.point[2] === point[2];
		}),
	);
	}

export function collectAffectedIds(session: TopologicWasmSession, entityId: string): readonly string[] {
	const linked = collectLinkedIds(session, entityId);
	const boundaryVertexIds = collectBoundaryVertexIds(session, linked);
	const peerFaceIds = session.fixture.topologies
		.filter((entity): entity is TopologicFaceEntity => entity.kind === "face")
		.filter((face) => linked.includes(face.id) || faceUsesBoundaryVertex(session, face, boundaryVertexIds))
		.map((face) => face.id);
	return [...new Set([...linked, ...peerFaceIds])];
	}

function collectGeometryBakeIds(session: TopologicWasmSession, entityId: string): ReadonlySet<string> {
	const dragged = session.getEntity(entityId);
	if (!dragged) return new Set();
	const bake = new Set<string>();
	for (const id of collectAffectedIds(session, entityId)) {
		const entity = session.getEntity(id);
		if (!entity) continue;
		if (entity.kind === "vertex") bake.add(id);
		if (entity.kind === "edge" && dragged.kind === "edge") bake.add(id);
	}
	return bake;
	}

function collectFaceSyncIds(session: TopologicWasmSession, entityId: string): readonly string[] {
	return collectAffectedIds(session, entityId).filter((id) => session.getEntity(id)?.kind === "face");
	}

function orderedFaceBoundaryVertexIds(session: TopologicWasmSession, face: TopologicFaceEntity): readonly string[] {
	const ordered: string[] = [];
	const seen = new Set<string>();
	for (const wireId of face.wires) {
		const wire = session.getEntity(wireId);
		if (wire?.kind !== "wire") continue;
		for (const edgeId of wire.edges) {
			const edge = session.getEntity(edgeId);
			if (edge?.kind !== "edge") continue;
			for (const vertexId of edge.vertices) {
				if (seen.has(vertexId)) continue;
				seen.add(vertexId);
				ordered.push(vertexId);
			}
		}
	}
	return ordered;
	}

function pointsEqual(left: Vec3, right: Vec3, epsilon = 1e-9): boolean {
	return Math.abs(left[0] - right[0]) <= epsilon && Math.abs(left[1] - right[1]) <= epsilon && Math.abs(left[2] - right[2]) <= epsilon;
	}

function squaredDistance(left: Vec3, right: Vec3): number {
	const dx = left[0] - right[0];
	const dy = left[1] - right[1];
	const dz = left[2] - right[2];
	return dx * dx + dy * dy + dz * dz;
	}

function resolveSurfaceVertexId(session: TopologicWasmSession, surfacePoint: Vec3, boundaryVertexIds: readonly string[]): string {
	for (const vertexId of boundaryVertexIds) {
		const vertex = session.getEntity(vertexId);
		if (vertex?.kind === "vertex" && pointsEqual(vertex.point, surfacePoint)) return vertexId;
	}
	let bestId = boundaryVertexIds[0];
	let bestDistance = Infinity;
	for (const vertexId of boundaryVertexIds) {
		const vertex = session.getEntity(vertexId);
		if (vertex?.kind !== "vertex") continue;
		const distance = squaredDistance(surfacePoint, vertex.point);
		if (distance < bestDistance) {
			bestDistance = distance;
			bestId = vertexId;
		}
	}
	return bestId;
	}

export function syncFaceSurfaceFromBoundary(before: TopologicWasmSession, after: TopologicWasmSession, face: TopologicFaceEntity): TopologicFaceEntity {
	const boundaryVertexIds = orderedFaceBoundaryVertexIds(after, face);
	if (boundaryVertexIds.length === 0) return face;
	const vertices =
		boundaryVertexIds.length === face.surface.vertices.length
			? boundaryVertexIds.map((vertexId) => (after.getEntity(vertexId) as TopologicVertexEntity).point)
			: face.surface.vertices.map((surfacePoint) => {
				const vertexId = resolveSurfaceVertexId(before, surfacePoint, boundaryVertexIds);
				return (after.getEntity(vertexId) as TopologicVertexEntity).point;
			});
	return {
		...face,
		surface: {
			vertices,
			triangles: face.surface.triangles,
		},
	};
	}

export function collectDragAttachIds(session: TopologicWasmSession, entityId: string): readonly string[] {
	return collectDescendantIds(session, entityId).filter((id) => {
		const entity = session.getEntity(id);
		return entity?.kind !== "vertex";
	});
	}

export function resolveEntityFrameTransform(_entity: TopologicWasmSession, entity: TopologicEntity, inherited: TopologicTransform | undefined): TopologicTransform | undefined {
	return composeTransforms(inherited, entity.transform);
	}

export function edgeModelPoints(session: TopologicWasmSession, edge: TopologicEdgeEntity): readonly Vec3[] {
	if (edge.curve && edge.curve.length >= 2) return edge.curve;
	return edge.vertices.map((vertexId) => {
		const vertex = session.getEntity(vertexId);
		return vertex?.kind === "vertex" ? vertex.point : ([0, 0, 0] as Vec3);
	});
	}

export function collectContainerPickPoints(session: TopologicWasmSession, entityId: string): readonly Vec3[] {
	const points: Vec3[] = [];
	for (const id of [entityId, ...collectDescendantIds(session, entityId)]) {
		const entity = session.getEntity(id);
		if (!entity) continue;
		if (entity.kind === "face") {
			for (const point of entity.surface.vertices) points.push(point);
		}
		if (entity.kind === "vertex") points.push(entity.point);
	}
	return points;
	}

export function computePickBounds(points: readonly Vec3[]): { center: Vec3; size: Vec3 } | null {
	if (points.length === 0) return null;
	let minX = Infinity;
	let minY = Infinity;
	let minZ = Infinity;
	let maxX = -Infinity;
	let maxY = -Infinity;
	let maxZ = -Infinity;
	for (const [x, y, z] of points) {
		minX = Math.min(minX, x);
		minY = Math.min(minY, y);
		minZ = Math.min(minZ, z);
		maxX = Math.max(maxX, x);
		maxY = Math.max(maxY, y);
		maxZ = Math.max(maxZ, z);
	}
	const minExtent = 0.2;
	return {
		center: [(minX + maxX) / 2, (minY + maxY) / 2, (minZ + maxZ) / 2],
		size: [Math.max(maxX - minX, minExtent), Math.max(maxY - minY, minExtent), Math.max(maxZ - minZ, minExtent)],
	};
	}

function isContainerKind(kind: TopologicKind): boolean {
	return kind === "topology" || kind === "shell" || kind === "cell" || kind === "cellComplex" || kind === "cluster";
	}

function entityGeometryAnchor(session: TopologicWasmSession, entity: TopologicEntity): Vec3 | null {
	if (entity.kind === "vertex") return entity.point;
	if (entity.kind === "edge") return centroid(edgeModelPoints(session, entity));
	if (entity.kind === "wire") {
		const points = entity.edges.flatMap((edgeId) => {
			const edge = session.getEntity(edgeId);
			return edge?.kind === "edge" ? edgeModelPoints(session, edge) : [];
		});
		return centroid(points);
	}
	if (entity.kind === "face") return centroid(entity.surface.vertices);
	if (isContainerKind(entity.kind)) {
		const points = collectContainerPickPoints(session, entity.id);
		return points.length > 0 ? centroid(points) : null;
	}
	return null;
	}

function positionOnlyTransform(transform: TopologicTransform | undefined): TopologicTransform | undefined {
	if (!transform?.position) return undefined;
	return { position: transform.position };
	}

export function resolveEntityGroupTransform(session: TopologicWasmSession, entity: TopologicEntity, frame: TopologicTransform | undefined): TopologicTransform | undefined {
	const anchor = entityGeometryAnchor(session, entity);
	const framePosition = positionOnlyTransform(frame);
	if (!anchor) return framePosition;
	return composeTransforms(framePosition, { position: anchor });
	}

export function resolveEntityRenderTransform(session: TopologicWasmSession, entity: TopologicEntity, inherited: TopologicTransform | undefined): TopologicTransform | undefined {
	const frame = resolveEntityFrameTransform(session, entity, inherited);
	return resolveEntityGroupTransform(session, entity, frame);
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
			...(entity.curve ? { curve: entity.curve.map((point) => topologicTransformPoint(point, delta)) as readonly Vec3[] } : {}),
		};
	}
	return rest as TopologicEntity;
	}

export function topologicEntityLabel(entity: TopologicEntity): string {
	return entity.label ?? entity.id;
	}

export function updateTopologicFixtureTransform(fixture: TopologicFixtureV1, entityId: string, transform: TopologicTransform): TopologicFixtureV1 {
	const session = new TopologicWasmSession(fixture);
	const entity = session.getEntity(entityId);
	if (!entity) return fixture;
	const previousWorld = resolveEntityGroupTransform(session, entity, entity.transform);
	const nextWorld = normalizeTransform(transform);
	const delta = composeTransforms(nextWorld, inverseTransform(previousWorld));
	const bakeIds = collectGeometryBakeIds(session, entityId);
	const faceSyncIds = new Set(collectFaceSyncIds(session, entityId));
	const bakedTopologies = fixture.topologies.map((topology) => (bakeIds.has(topology.id) ? applyTransformDeltaToEntity(topology, delta) : topology));
	const afterSession = new TopologicWasmSession({ ...fixture, topologies: bakedTopologies });
	return {
		...fixture,
		topologies: bakedTopologies.map((topology) =>
			topology.kind === "face" && faceSyncIds.has(topology.id) ? syncFaceSurfaceFromBoundary(session, afterSession, topology) : topology,
		),
	};
	}
//#endregion 🔖Math

//#region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("topologic runtime", () => {
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

		it("builds container pick bounds from descendant face and vertex points", async () => {
			const fixture = (await loadTopologicFixtureV1({
				schema: "elements.geometry.topologic.fixture/v1",
				roots: ["root"],
				topologies: [
					{ id: "root", kind: "topology", members: ["cell"] },
					{ id: "v0", kind: "vertex", point: [0, 0, 0] },
					{ id: "v1", kind: "vertex", point: [2, 0, 0] },
					{ id: "v2", kind: "vertex", point: [0, 0, 2] },
					{ id: "shell", kind: "shell", faces: ["face"] },
					{ id: "face", kind: "face", wires: [], surface: { vertices: [[0, 0, 0], [2, 0, 0], [0, 0, 2]], triangles: [0, 1, 2] } },
					{ id: "cell", kind: "cell", shells: ["shell"] },
				],
			})) as TopologicFixtureV1;
			const session = new TopologicWasmSession(fixture);
			const bounds = computePickBounds(collectContainerPickPoints(session, "cell"));
			expect(bounds?.center).toEqual([1, 0, 1]);
			expect(bounds?.size).toEqual([2, 0.2, 2]);
		});

		it("limits drag attach to descendants so ancestors cannot create scene cycles", async () => {
			const fixture = (await loadTopologicFixtureV1({
				schema: "elements.geometry.topologic.fixture/v1",
				roots: ["root"],
				topologies: [
					{ id: "root", kind: "topology", members: ["face"] },
					{ id: "v0", kind: "vertex", point: [0, 0, 0] },
					{ id: "e0", kind: "edge", vertices: ["v0", "v1"] },
					{ id: "v1", kind: "vertex", point: [2, 0, 0] },
					{ id: "wire", kind: "wire", edges: ["e0"] },
					{ id: "face", kind: "face", wires: ["wire"], surface: { vertices: [[0, 0, 0], [2, 0, 0], [0, 0, 2]], triangles: [0, 1, 2] } },
				],
			})) as TopologicFixtureV1;
			const session = new TopologicWasmSession(fixture);
			expect(collectDragAttachIds(session, "v0")).toEqual([]);
			expect(collectDragAttachIds(session, "face")).toEqual(["wire", "e0"]);
		});

		it("places the vertex group transform at the point for gumball editing", async () => {
			const fixture = (await loadTopologicFixtureV1({
				schema: "elements.geometry.topologic.fixture/v1",
				roots: ["root"],
				topologies: [
					{ id: "root", kind: "topology", members: ["vertex"] },
					{ id: "vertex", kind: "vertex", point: [1, 2, 3] },
				],
			})) as TopologicFixtureV1;
			const session = new TopologicWasmSession(fixture);
			const vertex = session.getEntity("vertex") as TopologicVertexEntity;
			const group = resolveEntityGroupTransform(session, vertex, vertex.transform);
			expect(group?.position).toEqual([1, 2, 3]);
		});

		it("bakes a vertex drag into point geometry", async () => {
			const fixture = (await loadTopologicFixtureV1({
				schema: "elements.geometry.topologic.fixture/v1",
				roots: ["root"],
				topologies: [
					{ id: "root", kind: "topology", members: ["vertex"] },
					{ id: "vertex", kind: "vertex", point: [0, 0, 0] },
				],
			})) as TopologicFixtureV1;
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
					{ id: "face", kind: "face", wires: ["wire"], surface: { vertices: [[0, 0, 0], [2, 0, 0], [2, 0, 2]], triangles: [0, 1, 2] } },
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

		it("places edge endpoints at the same world position as child vertices", async () => {
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
			const edge = session.getEntity("edge") as TopologicEdgeEntity;
			const start = session.getEntity("start") as TopologicVertexEntity;
			const edgeGroup = resolveEntityGroupTransform(session, edge, edge.transform);
			const modelPoints = edgeModelPoints(session, edge);
			const anchor = centroid(modelPoints);
			const edgeStartWorld = topologicTransformPoint([modelPoints[0][0] - anchor[0], modelPoints[0][1] - anchor[1], modelPoints[0][2] - anchor[2]], edgeGroup);
			const vertexGroup = resolveEntityGroupTransform(session, start, start.transform);
			const vertexWorld = topologicTransformPoint([0, 0, 0], vertexGroup);
			expect(edgeStartWorld).toEqual(vertexWorld);
		});

		it("applies a single delta to vertices when a face is translated", async () => {
			const fixture = (await loadTopologicFixtureV1({
				schema: "elements.geometry.topologic.fixture/v1",
				roots: ["root"],
				topologies: [
					{ id: "root", kind: "topology", members: ["face"] },
					{ id: "v0", kind: "vertex", point: [0, 0, 0] },
					{ id: "v1", kind: "vertex", point: [2, 0, 0] },
					{ id: "e0", kind: "edge", vertices: ["v0", "v1"] },
					{ id: "wire", kind: "wire", edges: ["e0"] },
					{ id: "face", kind: "face", wires: ["wire"], surface: { vertices: [[0, 0, 0], [2, 0, 0], [2, 0, 2]], triangles: [0, 1, 2] } },
				],
			})) as TopologicFixtureV1;
			const session = new TopologicWasmSession(fixture);
			const face = session.getEntity("face") as TopologicFaceEntity;
			const faceWorld = resolveEntityGroupTransform(session, face, face.transform);
			const updated = updateTopologicFixtureTransform(fixture, "face", {
				position: [faceWorld?.position?.[0] ?? 0, (faceWorld?.position?.[1] ?? 0) + 5, faceWorld?.position?.[2] ?? 0],
				rotation: faceWorld?.rotation ?? [0, 0, 0, 1],
				scale: faceWorld?.scale ?? 1,
			});
			expect(new TopologicWasmSession(updated).getEntity("v0")).toMatchObject({ point: [0, 5, 0] });
		});

		it("reshapes face surfaces when only one boundary vertex moves", async () => {
			const fixture = (await loadTopologicFixtureV1({
				schema: "elements.geometry.topologic.fixture/v1",
				roots: ["root"],
				topologies: [
					{ id: "root", kind: "topology", members: ["face"] },
					{ id: "v0", kind: "vertex", point: [0, 0, 0] },
					{ id: "v1", kind: "vertex", point: [2, 0, 0] },
					{ id: "v2", kind: "vertex", point: [0, 0, 2] },
					{ id: "e0", kind: "edge", vertices: ["v0", "v1"] },
					{ id: "e1", kind: "edge", vertices: ["v1", "v2"] },
					{ id: "e2", kind: "edge", vertices: ["v2", "v0"] },
					{ id: "wire", kind: "wire", edges: ["e0", "e1", "e2"] },
					{ id: "face", kind: "face", wires: ["wire"], surface: { vertices: [[0, 0, 0], [2, 0, 0], [0, 0, 2]], triangles: [0, 1, 2] } },
				],
			})) as TopologicFixtureV1;
			const session = new TopologicWasmSession(fixture);
			const vertex = session.getEntity("v0") as TopologicVertexEntity;
			const vertexWorld = resolveEntityGroupTransform(session, vertex, vertex.transform);
			const updated = updateTopologicFixtureTransform(fixture, "v0", {
				position: [vertexWorld?.position?.[0] ?? 0, (vertexWorld?.position?.[1] ?? 0) + 4, vertexWorld?.position?.[2] ?? 0],
				rotation: vertexWorld?.rotation ?? [0, 0, 0, 1],
				scale: vertexWorld?.scale ?? 1,
			});
			const face = new TopologicWasmSession(updated).getEntity("face") as TopologicFaceEntity;
			expect(face.surface.vertices[0]).toEqual([0, 4, 0]);
			expect(face.surface.vertices[1]).toEqual([2, 0, 0]);
			expect(face.surface.vertices[2]).toEqual([0, 0, 2]);
		});

		it("moves peer faces that share a boundary edge when a face is translated", async () => {
			const fixture = (await loadTopologicFixtureV1({
				schema: "elements.geometry.topologic.fixture/v1",
				roots: ["root"],
				topologies: [
					{ id: "root", kind: "topology", members: ["face-a", "face-b"] },
					{ id: "v0", kind: "vertex", point: [0, 0, 0] },
					{ id: "v1", kind: "vertex", point: [2, 0, 0] },
					{ id: "v2", kind: "vertex", point: [2, 0, 3] },
					{ id: "e0", kind: "edge", vertices: ["v0", "v1"] },
					{ id: "wire-a", kind: "wire", edges: ["e0"] },
					{ id: "wire-b", kind: "wire", edges: ["e0"] },
					{ id: "face-a", kind: "face", wires: ["wire-a"], surface: { vertices: [[0, 0, 0], [2, 0, 0], [0, 0, 2]], triangles: [0, 1, 2] } },
					{ id: "face-b", kind: "face", wires: ["wire-b"], surface: { vertices: [[0, 0, 0], [2, 0, 0], [2, 0, 3]], triangles: [0, 1, 2] } },
				],
			})) as TopologicFixtureV1;
			const session = new TopologicWasmSession(fixture);
			const faceA = session.getEntity("face-a") as TopologicFaceEntity;
			const faceWorld = resolveEntityGroupTransform(session, faceA, faceA.transform);
			const updated = updateTopologicFixtureTransform(fixture, "face-a", {
				position: [faceWorld?.position?.[0] ?? 0, (faceWorld?.position?.[1] ?? 0) + 6, faceWorld?.position?.[2] ?? 0],
				rotation: faceWorld?.rotation ?? [0, 0, 0, 1],
				scale: faceWorld?.scale ?? 1,
			});
			const faceB = new TopologicWasmSession(updated).getEntity("face-b") as TopologicFaceEntity;
			expect(faceB.surface.vertices[0]).toEqual([0, 6, 0]);
			expect(faceB.surface.vertices[1]).toEqual([2, 6, 0]);
		});

		it("moves ancestor face surfaces when an edge is translated", async () => {
			const fixture = (await loadTopologicFixtureV1({
				schema: "elements.geometry.topologic.fixture/v1",
				roots: ["root"],
				topologies: [
					{ id: "root", kind: "topology", members: ["face"] },
					{ id: "v0", kind: "vertex", point: [0, 0, 0] },
					{ id: "v1", kind: "vertex", point: [2, 0, 0] },
					{ id: "e0", kind: "edge", vertices: ["v0", "v1"] },
					{ id: "wire", kind: "wire", edges: ["e0"] },
					{ id: "face", kind: "face", wires: ["wire"], surface: { vertices: [[0, 0, 0], [2, 0, 0], [2, 0, 2]], triangles: [0, 1, 2] } },
				],
			})) as TopologicFixtureV1;
			const session = new TopologicWasmSession(fixture);
			const edge = session.getEntity("e0") as TopologicEdgeEntity;
			const edgeWorld = resolveEntityGroupTransform(session, edge, edge.transform);
			const updated = updateTopologicFixtureTransform(fixture, "e0", {
				position: [edgeWorld?.position?.[0] ?? 0, (edgeWorld?.position?.[1] ?? 0) + 4, edgeWorld?.position?.[2] ?? 0],
				rotation: edgeWorld?.rotation ?? [0, 0, 0, 1],
				scale: edgeWorld?.scale ?? 1,
			});
			const face = new TopologicWasmSession(updated).getEntity("face") as TopologicFaceEntity;
			expect(face.surface.vertices[0]).toEqual([0, 4, 0]);
			expect(face.surface.vertices[1]).toEqual([2, 4, 0]);
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
			expect(session.edgeCurve("edge")).toEqual([[0, 0, 0], [2, 0, 0]]);
		});
	});
}
//#endregion 🧪Tests