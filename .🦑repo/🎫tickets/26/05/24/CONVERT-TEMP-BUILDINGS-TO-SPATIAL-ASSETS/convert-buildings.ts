import { cast, fromBREP, getEdges, getFaces, getHashCode, init, isSameShape, iterTopo, unwrap, vertexPosition, verticesOfEdge, wiresOfFace } from "brepjs";
import type { AnyShape, Dimension } from "brepjs";
import { mkdir, readFile, writeFile } from "node:fs/promises";
import { dirname, join } from "node:path";

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

const root = new URL("../../../../../..", import.meta.url).pathname.replace(/^\/([A-Za-z]:\/)/, "$1");
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

function sortedRecords<T extends { id: string }>(records: Record<string, T>): Record<string, T> {
  return Object.fromEntries(Object.entries(records).sort(([a], [b]) => a.localeCompare(b)));
}

function topology(id: string, rootShape: Shape) {
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
    schema: "spatial.topology/v1",
    revision: 1,
    vertices: sortedRecords(vertices),
    edges: sortedRecords(edges),
    wires: sortedRecords(wires),
    faces: sortedRecords(faces),
    shells: sortedRecords(shells),
    cells: sortedRecords(cells),
    cellComplexes: { [complexId]: { id: complexId, cellIds } },
    clusters: { [clusterId]: { id: clusterId, memberIds: [complexId] } },
  };
}

await init();

for (const item of sources) {
  const sourcePath = join(root, "temp", item.source);
  const targetPath = join(root, "spatial", "fixtures", item.target);
  const sourceJson = JSON.parse(await readFile(sourcePath, "utf8")) as SourceBuilding[];
  const shape = unwrap(fromBREP(sourceJson[0]!.brep));
  const asset = topology(item.id, shape);

  await mkdir(dirname(targetPath), { recursive: true });
  await writeFile(targetPath, `${JSON.stringify(asset, null, 2)}\n`, "utf8");
  console.log(
    `[DEBUG] ${item.target}: ${Object.keys(asset.vertices).length} vertices, ${Object.keys(asset.edges).length} edges, ${Object.keys(asset.wires).length} wires, ${Object.keys(asset.faces).length} faces, ${Object.keys(asset.shells).length} shells, ${Object.keys(asset.cells).length} cells`,
  );
}
