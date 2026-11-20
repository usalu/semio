import type { Mesh } from "three";
import { GLTFLoader } from "three/examples/jsm/loaders/GLTFLoader.js";
import { STLLoader } from "three/examples/jsm/loaders/STLLoader.js";
import { OBJLoader } from "three/examples/jsm/loaders/OBJLoader.js";
import { SphereGeometry, TorusGeometry, ShapeUtils, Triangle, Vector2, Vector3 } from "three";
import retainedWorkbench from "../🧫️fixtures/🥽️retained-workbench/🔣️.json";
import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import Ajv from "ajv";
const channelContract = JSON.parse(readFileSync(new URL("../../../🧬️schema/🪪️channels/🔣️.json", import.meta.url), "utf8"));
const channelFixtures = JSON.parse(readFileSync(new URL("../../../🧪️tests/🧫️fixtures/🪪️channels/🔣️.json", import.meta.url), "utf8"));
test("packaged BRep channels satisfy typed schema and portable identities", () => {
  const descriptor = JSON.parse(readFileSync(new URL("../../../🧩️component/🔣️.json", import.meta.url), "utf8"));
  const validate = new Ajv().compile(channelContract);
  for (const topic of descriptor.manifest.topicContributions) {
    const manifest = JSON.parse(topic.payload.manifestJson);
    for (const operator of manifest.contributes.operators) for (const direction of ["inputs", "outputs"]) {
      const channels = operator[direction];
      for (const channel of channels) expect(validate(channel), `${operator.id} ${direction} ${channel.name}: ${JSON.stringify(validate.errors)}`).toBe(true);
      for (const key of ["code", "abbreviation", "name", "fullName"]) expect(new Set(channels.map((channel: Record<string, unknown>) => channel[key])).size, `${operator.id} ${direction} ${key}`).toBe(channels.length);
    }
    for (const fixture of channelFixtures.cases) {
      const operator = manifest.contributes.operators.find((operator: { id: string }) => operator.id === fixture.operator);
      for (const [key, fields] of Object.entries(fixture.expected)) {
        const channel = operator[fixture.direction].find((channel: { name: string }) => channel.name === key);
        for (const [field, expected] of Object.entries(fields as object)) expect(channel[field]).toEqual(expected);
      }
    }
    for (const kind of ["Vertex", "Edge", "Face"]) expect(manifest.contributes.operators.some((operator: { id: string }) => operator.id === `brep.mesh.inspect${kind}`)).toBe(true);
  }
});

test("the retained workbench measures agree with independent polygon geometry", () => {
  const side = 1 - 2 * retainedWorkbench.amount;
  const patch = [[0, 0], [side, 0], [side, side], [0, side]].map(([x, y]) => new Vector3(x, y, 0));
  const patchTriangles = [new Triangle(patch[0]!, patch[1]!, patch[2]!), new Triangle(patch[0]!, patch[2]!, patch[3]!)];
  const patchArea = patchTriangles.reduce((sum, triangle) => sum + triangle.getArea(), 0);
  const perimeter = patch.reduce((sum, point, index) => sum + point.distanceTo(patch[(index + 1) % 4]!), 0);
  expect(6 + perimeter * retainedWorkbench.distance).toBeCloseTo(retainedWorkbench.area, 12);
  expect(1 + patchArea * retainedWorkbench.distance).toBeCloseTo(retainedWorkbench.volume, 12);

  const dissolve = retainedWorkbench.vertexDissolve;
  const polygon = dissolve.boundary.map((vertex) => {
    const point = dissolve.mesh.vertices[vertex]!;
    return new Vector2(point[0], point[1]);
  });
  expect(Math.abs(ShapeUtils.area(polygon))).toBe(dissolve.area);

  const { inspection, mesh } = dissolve;
  const point = (vertex: number) => new Vector3(...mesh.vertices[vertex]! as [number, number, number]);
  const face = mesh.faces[inspection.face]!;
  const [a, b, c] = face.map(point);
  const center = face.reduce((sum, vertex) => sum.add(point(vertex)), new Vector3()).divideScalar(face.length);
  expect(point(inspection.vertex).toArray()).toEqual(inspection.position);
  expect(a!.toArray()).toEqual(inspection.start);
  expect(b!.toArray()).toEqual(inspection.end);
  expect(a!.distanceTo(b!)).toBe(inspection.length);
  expect(face).toEqual(inspection.vertices);
  new Triangle(a!, b!, c!).getNormal(new Vector3()).toArray().forEach((axis, index) => expect(axis).toBeCloseTo(inspection.normal[index]!, 12));
  expect(center.toArray()).toEqual(inspection.center);
});

test("the OBJ workbench specimen imports as a polygon with the independent OBJLoader surface", () => {
  const fixture = retainedWorkbench.objImport;
  const loaded = new OBJLoader().parse(fixture.text);
  const { geometry } = loaded.children[0] as Mesh;
  const positions = geometry.getAttribute("position");
  const coordinates = new Set<string>();
  let area = 0;
  for (let index = 0; index < positions.count; index += 3) {
    const points = [index, index + 1, index + 2].map((offset) => new Vector3().fromBufferAttribute(positions, offset));
    points.forEach((point) => coordinates.add(point.toArray().join(",")));
    area += new Triangle(points[0]!, points[1]!, points[2]!).getArea();
  }
  expect(coordinates.size).toBe(fixture.vertices);
  expect(area).toBe(fixture.area);
  expect(fixture.text.split("\n").filter((line) => line.startsWith("f "))).toHaveLength(fixture.faces);
});

test("native mesh surface samples satisfy the polygon schema and Three.js normal geometry", () => {
  const root = new URL("../../../../../../../../", import.meta.url);
  const source = JSON.parse(readFileSync(new URL("🧰️framework/🔨️modules/🏗️mesh-engine/🧬️schema/🥽️polygon/🔣️.json", root), "utf8"));
  const validate = new Ajv().compile(source);
  const { polygon, normal, seam } = retainedWorkbench.surfacePipeline;
  expect(validate(polygon), JSON.stringify(validate.errors)).toBe(true);
  const invalid = JSON.parse(JSON.stringify(polygon));
  invalid.surface.push({ kind: "cornerUv", index: 0, uv: [1] });
  expect(validate(invalid)).toBe(false);
  const points = polygon.faces[0]!.map((id) => new Vector3(...polygon.vertices[id]! as [number, number, number]));
  new Triangle(points[0]!, points[1]!, points[2]!).getNormal(new Vector3()).toArray().forEach((value, index) => expect(value).toBeCloseTo(normal[index]!, 12));
  const [a, b] = [points[seam]!, points[(seam + 1) % points.length]!];
  expect(a.distanceTo(b)).toBe(1);
});

test("binary STL and GLB workbench geometry agrees with the independent Three.js loaders", async () => {
  const root = new URL("../../../../../../../../", import.meta.url);
  const catalogue = new URL("✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📐️geometry/📼️mesh-interchange/🧪️tests/🧫️fixtures/🔣️.json", root);
  const { cases } = JSON.parse(readFileSync(catalogue, "utf8"));
  const law = retainedWorkbench.binaryInterchange;
  for (const { suffix } of law.formats) {
    const { inputs } = cases.find((row: { kind: string; name: string }) => row.kind === `mesh.interchange.import${suffix}` && row.name.startsWith("import the cube"));
    const bytes = Uint8Array.from(Buffer.from(inputs.data, "base64")).buffer;
    const geometry = suffix === "Stl"
      ? new STLLoader().parse(bytes)
      : (await new GLTFLoader().parseAsync(bytes, "")).scene.children[0] as Mesh;
    const loaded = "geometry" in geometry ? geometry.geometry : geometry;
    const positions = loaded.getAttribute("position");
    const indices = loaded.getIndex();
    const corners = indices?.count ?? positions.count;
    const vertices = new Set<string>();
    let area = 0;
    let volume = 0;
    for (let i = 0; i < corners; i += 3) {
      const a = new Vector3().fromBufferAttribute(positions, indices?.getX(i) ?? i);
      const b = new Vector3().fromBufferAttribute(positions, indices?.getX(i + 1) ?? i + 1);
      const c = new Vector3().fromBufferAttribute(positions, indices?.getX(i + 2) ?? i + 2);
      for (const point of [a, b, c]) vertices.add(point.toArray().join(","));
      area += new Triangle(a, b, c).getArea();
      volume += a.dot(new Vector3().crossVectors(b, c)) / 6;
    }
    expect(corners / 3).toBe(law.faces);
    expect(vertices.size).toBe(law.vertices);
    expect(area).toBeCloseTo(law.area, 12);
    expect(volume).toBeCloseTo(law.volume, 12);
  }
});

test("parametric mesh topology matches the indexed Three.js primitives", () => {
  for (const fixture of retainedWorkbench.parametricPrimitives) {
    const { major, minor, radius, rings, segments } = fixture.inputs;
    const geometry = fixture.operator === "brep.mesh.torus"
      ? new TorusGeometry(major, minor, rings, segments)
      : new SphereGeometry(radius, segments, rings);
    const vertices = new Set<string>();
    const position = geometry.getAttribute("position");
    for (let index = 0; index < position.count; index++) {
      vertices.add([position.getX(index), position.getY(index), position.getZ(index)]
        .map(axis => Math.round(axis * 1e6)).join(","));
    }
    expect(vertices.size).toBe(fixture.vertices);
    expect(geometry.index!.count / 3).toBe(fixture.triangles);
    geometry.dispose();
  }
});
