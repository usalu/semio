import { init, fromBREP, mesh, unwrap } from "brepjs";
import { mkdir, readFile, writeFile } from "node:fs/promises";
import { dirname, join } from "node:path";

type Vec3 = [number, number, number];

interface SourceBuilding {
	brep: string;
}

const root = new URL("../../../../../..", import.meta.url).pathname.replace(/^\/([A-Za-z]:\/)/, "$1");
const sources = [
	{ source: "Small_Building.json", target: "small-building.topology.json", id: "small-building", label: "Small Building" },
	{ source: "Tall_Building.json", target: "tall-building.topology.json", id: "tall-building", label: "Tall Building" },
	{ source: "Large_Building.json", target: "large-building.topology.json", id: "large-building", label: "Large Building" },
] as const;

function round(n: number): number {
	return Number(n.toFixed(6));
}

function vec(values: ArrayLike<number>, index: number): Vec3 {
	return [round(values[index]!), round(values[index + 1]!), round(values[index + 2]!)];
}

function topology(id: string, label: string, positions: Float32Array, indices: Uint32Array) {
	const vertices: Record<string, { id: string; position: Vec3 }> = {};
	const meshVertices: Vec3[] = [];
	const triangles: [number, number, number][] = [];

	for (let index = 0; index < positions.length; index += 3) {
		const vertexId = `${id}-v${index / 3}`;
		const position = vec(positions, index);
		vertices[vertexId] = { id: vertexId, position };
		meshVertices.push(position);
	}

	for (let index = 0; index < indices.length; index += 3) {
		triangles.push([indices[index]!, indices[index + 1]!, indices[index + 2]!]);
	}

	const wireId = `${id}-mesh-wire`;
	const faceId = `${id}-mesh-face`;
	const shellId = `${id}-shell`;
	const cellId = `${id}-cell`;
	const complexId = `${id}-complex`;
	const clusterId = `${id}-asset`;

	return {
		schema: "spatial.topology/v1",
		revision: 1,
		vertices,
		edges: {},
		wires: {
			[wireId]: { id: wireId, edgeIds: [], closed: true },
		},
		faces: {
			[faceId]: {
				id: faceId,
				outerWireId: wireId,
				surface: { kind: "mesh", vertices: meshVertices, triangles },
			},
		},
		shells: {
			[shellId]: { id: shellId, faceIds: [faceId] },
		},
		cells: {
			[cellId]: { id: cellId, shellIds: [shellId] },
		},
		cellComplexes: {
			[complexId]: { id: complexId, cellIds: [cellId], sharedFaceIds: [] },
		},
		clusters: {
			[clusterId]: { id: clusterId, memberIds: [complexId] },
		},
	};
}

await init();

for (const item of sources) {
	const sourcePath = join(root, "temp", item.source);
	const targetPath = join(root, "spatial", "fixtures", item.target);
	const sourceJson = JSON.parse(await readFile(sourcePath, "utf8")) as SourceBuilding[];
	const shape = unwrap(fromBREP(sourceJson[0]!.brep));
	const shapeMesh = mesh(shape, { tolerance: 0.5, angularTolerance: 0.35, skipNormals: true, cache: false });
	const asset = topology(item.id, item.label, shapeMesh.vertices, shapeMesh.triangles);

	await mkdir(dirname(targetPath), { recursive: true });
	await writeFile(targetPath, `${JSON.stringify(asset, null, 2)}\n`, "utf8");
	console.log(
		`[DEBUG] ${item.target}: ${shapeMesh.vertices.length / 3} vertices, ${shapeMesh.triangles.length / 3} triangles`,
	);
}
