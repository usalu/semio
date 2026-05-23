#!/usr/bin/env bun
/** @emoji 🏢 Converts temp/*_Building.json BREPs to spatial.topology/v1 fixtures (centered, array buckets). */
import {
	cast,
	fromBREP,
	getEdges,
	getFaces,
	getHashCode,
	init,
	isSameShape,
	iterTopo,
	unwrap,
	vertexPosition,
	verticesOfEdge,
	wiresOfFace,
} from "brepjs";
import type { AnyShape, Dimension } from "brepjs";
import { mkdir, readFile, writeFile } from "node:fs/promises";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

type Vec3 = [number, number, number];
type Shape = AnyShape<Dimension>;

interface SourceBuilding {
	brep: string;
}

interface EntitySlot {
	id: string;
	shape: Shape;
}

interface IdScope {
	prefix: string;
	slots: Map<string, EntitySlot[]>;
}

interface TopologyBuckets {
	vertices: { id: string; position: Vec3 }[];
	edges: { id: string; vertexIds: string[] }[];
	wires: { id: string; edgeIds: string[] }[];
	faces: { id: string; wireIds: string[] }[];
	shells: { id: string; faceIds: string[] }[];
	cells: { id: string; shellIds: string[] }[];
	cellComplexes: { id: string; cellIds: string[] }[];
	clusters: { id: string; memberIds: string[] }[];
}

const repoRoot = join(fileURLToPath(new URL("../../../../../..", import.meta.url)));
const sources = [
	{ source: "Small_Building.json", target: "small-building.topology.json", id: "small-building" },
	{ source: "Tall_Building.json", target: "tall-building.topology.json", id: "tall-building" },
	{ source: "Large_Building.json", target: "large-building.topology.json", id: "large-building" },
] as const;

function round(n: number): number {
	return Number(n.toFixed(6));
}

function vec(values: readonly number[]): Vec3 {
	return [round(values[0]!), round(values[1]!), round(values[2]!)];
}

function castTopo(shape: unknown): Shape {
	return unwrap(cast(shape as never));
}

function idFor(scope: IdScope, kind: string, shape: Shape): string {
	const hash = String(getHashCode(shape));
	const slots = scope.slots.get(hash) ?? [];
	const existing = slots.find((slot) => isSameShape(slot.shape, shape));
	if (existing) return existing.id;
	const id = `${scope.prefix}-${kind}-${hash}${slots.length ? `-${slots.length}` : ""}`;
	slots.push({ id, shape });
	scope.slots.set(hash, slots);
	return id;
}

function sortById<T extends { id: string }>(records: T[]): T[] {
	return [...records].sort((a, b) => a.id.localeCompare(b.id));
}

function topologyFromBrep(id: string, rootShape: Shape): TopologyBuckets {
	const scope: IdScope = { prefix: id, slots: new Map() };
	const vertices: Record<string, { id: string; position: Vec3 }> = {};
	const edges: Record<string, { id: string; vertexIds: string[] }> = {};
	const wires: Record<string, { id: string; edgeIds: string[] }> = {};
	const faces: Record<string, { id: string; wireIds: string[] }> = {};
	const shells: Record<string, { id: string; faceIds: string[] }> = {};
	const cells: Record<string, { id: string; shellIds: string[] }> = {};

	function visitVertex(vertex: Shape): string {
		const vertexId = idFor(scope, "vertex", vertex);
		vertices[vertexId] ??= { id: vertexId, position: vec(vertexPosition(vertex as never)) };
		return vertexId;
	}

	function visitEdge(edge: Shape): string {
		const edgeId = idFor(scope, "edge", edge);
		edges[edgeId] ??= {
			id: edgeId,
			vertexIds: verticesOfEdge(edge as never).map((vertex) => visitVertex(vertex as Shape)),
		};
		return edgeId;
	}

	function visitWire(wire: Shape): string {
		const wireId = idFor(scope, "wire", wire);
		wires[wireId] ??= { id: wireId, edgeIds: getEdges(wire).map((edge) => visitEdge(edge as Shape)) };
		return wireId;
	}

	function visitFace(face: Shape): string {
		const faceId = idFor(scope, "face", face);
		faces[faceId] ??= { id: faceId, wireIds: wiresOfFace(face as never).map((wire) => visitWire(wire as Shape)) };
		return faceId;
	}

	function visitShell(shell: Shape): string {
		const shellId = idFor(scope, "shell", shell);
		shells[shellId] ??= { id: shellId, faceIds: getFaces(shell).map((face) => visitFace(face as Shape)) };
		return shellId;
	}

	function visitCell(cell: Shape): string {
		const cellId = idFor(scope, "cell", cell);
		cells[cellId] ??= {
			id: cellId,
			shellIds: [...iterTopo(cell.wrapped, "shell")].map((shell) => visitShell(castTopo(shell))),
		};
		return cellId;
	}

	const cellIds = [...iterTopo(rootShape.wrapped, "solid")].map((cell) => visitCell(castTopo(cell)));
	const complexId = `${id}-cell-complex`;
	const clusterId = `${id}-asset`;

	return {
		vertices: sortById(Object.values(vertices)),
		edges: sortById(Object.values(edges)),
		wires: sortById(Object.values(wires)),
		faces: sortById(Object.values(faces)),
		shells: sortById(Object.values(shells)),
		cells: sortById(Object.values(cells)),
		cellComplexes: [{ id: complexId, cellIds: sortById(cellIds.map((cid) => ({ id: cid }))).map((x) => x.id) }],
		clusters: [{ id: clusterId, memberIds: [complexId] }],
	};
}

function centroid(vertices: readonly { position: Vec3 }[]): Vec3 {
	if (!vertices.length) return [0, 0, 0];
	let sx = 0;
	let sy = 0;
	let sz = 0;
	for (const v of vertices) {
		sx += v.position[0];
		sy += v.position[1];
		sz += v.position[2];
	}
	const n = vertices.length;
	return [round(sx / n), round(sy / n), round(sz / n)];
}

function centerBuckets(buckets: TopologyBuckets): TopologyBuckets {
	const c = centroid(buckets.vertices);
	return {
		...buckets,
		vertices: buckets.vertices.map((v) => ({
			id: v.id,
			position: [round(v.position[0] - c[0]), round(v.position[1] - c[1]), round(v.position[2] - c[2])] as Vec3,
		})),
	};
}

await init();

for (const item of sources) {
	const sourcePath = join(repoRoot, "temp", item.source);
	const targetPath = join(repoRoot, "spatial", "fixtures", item.target);
	const sourceJson = JSON.parse(await readFile(sourcePath, "utf8")) as SourceBuilding[];
	const shape = unwrap(fromBREP(sourceJson[0]!.brep));
	const centered = centerBuckets(topologyFromBrep(item.id, shape));
	const asset = {
		schema: "spatial.topology/v1" as const,
		revision: 1,
		...centered,
	};

	await mkdir(dirname(targetPath), { recursive: true });
	await writeFile(targetPath, `${JSON.stringify(asset, null, 2)}\n`, "utf8");
	console.log(
		`[DEBUG] ${item.target}: ${asset.vertices.length} vertices, ${asset.edges.length} edges, ${asset.cells.length} cells, cellComplex=${asset.cellComplexes[0]?.cellIds.length ?? 0}`,
	);
}
