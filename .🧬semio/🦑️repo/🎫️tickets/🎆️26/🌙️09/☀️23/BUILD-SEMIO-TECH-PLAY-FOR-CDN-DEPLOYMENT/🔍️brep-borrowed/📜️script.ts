import Ajv from "ajv";
import { strict as assert } from "node:assert";
import { existsSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { OBJLoader } from "three/examples/jsm/loaders/OBJLoader.js";

function oracle(): void {
  let root = import.meta.dirname;
  while (!existsSync(join(root, "nx.json"))) {
    const parent = dirname(root);
    assert.notEqual(parent, root);
    root = parent;
  }
  const brep = join(root, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep");
  const binary = join(brep, "🚪️io/💾️binary/🧬️mutations");
  const inputs = join(binary, "🫳️borrowed/🧪️tests/🧫️fixtures");
  const fixture = JSON.parse(readFileSync(join(inputs, "🔣️.json"), "utf8"));
  const read = (path: string) => JSON.parse(readFileSync(path, "utf8"));
  const ajv = new Ajv({ strict: false, allErrors: true });
  ajv.addSchema(read(join(brep, "🧬️schema/💡️inferences/🔣️.json")));
  ajv.addSchema(read(join(brep, "🧬️schema/📸️snapshot/🔣️.json")));
  const validate = ajv.compile(read(join(inputs, "🧬️schema/🔣️.json")));
  assert.ok(validate(fixture), JSON.stringify(validate.errors));
  assert.equal(validate({ ...fixture, header: [1, 14] }), false);
  assert.equal(validate({ ...fixture, maximumDepth: 65 }), false);
  assert.equal(validate({ ...fixture, snapshot: { ...fixture.snapshot, edges: [{ ...fixture.snapshot.edges[0], curve: { kind: "unknown" } }] } }), false);
  assert.deepEqual(Object.keys(fixture.snapshot), fixture.rootKeys);
  assert.deepEqual(fixture.snapshot.edges.map((edge: any) => edge.curve.kind), fixture.curveKinds);
  assert.deepEqual(fixture.snapshot.coedges.filter((coedge: any) => coedge.pcurve).map((coedge: any) => coedge.pcurve.kind), fixture.curve2Kinds);
  assert.deepEqual(fixture.snapshot.faces.map((face: any) => face.surface.kind), fixture.surfaceKinds);
  const imported = new OBJLoader().parse(fixture.importedObj.text);
  const vertices = new Set<string>(), edges = new Map<string, number>();
  let triangles = 0;
  imported.traverse((child: any) => {
    if (!child.geometry) return;
    const position = child.geometry.attributes.position;
    assert.equal(position.count % 3, 0);
    for (let index = 0; index < position.count; index += 3) {
      const points = [0, 1, 2].map(offset => [position.getX(index + offset), position.getY(index + offset), position.getZ(index + offset)].join(","));
      points.forEach(point => vertices.add(point));
      for (let offset = 0; offset < 3; offset++) {
        const key = [points[offset]!, points[(offset + 1) % 3]!].sort().join("/");
        edges.set(key, (edges.get(key) ?? 0) + 1);
      }
      triangles++;
    }
  });
  assert.equal(vertices.size, fixture.importedObj.counts.vertices);
  assert.equal(edges.size, fixture.importedObj.counts.edges);
  assert.equal(triangles, fixture.importedObj.counts.faces);
  assert.ok([...edges.values()].every(uses => uses === 2));
  console.log("[DEBUG] independent THREE OBJLoader imported4vertices6sharedEdges4faces closed12uses");
  const raw = Buffer.from(JSON.stringify(fixture.snapshot));
  const wire = Buffer.concat([Buffer.from(fixture.header), Buffer.from(fixture.prefix), Buffer.from(raw.toString("hex"))]);
  assert.deepEqual(Buffer.from(wire.subarray(11).toString(), "hex"), raw);
  assert.equal(Buffer.from(wire.subarray(11).toString(), "hex").toString(), JSON.stringify(fixture.snapshot));
  console.log("[DEBUG] independent Ajv/Node UTF8 hex oracle nine ordered root fields, four curves, four pcurves, six surfaces; wire header1/13 snapshot=; typed serde exact-byte native witness is separate");
  const source = readFileSync(join(binary, "🦀️.rs"), "utf8");
  assert.match(source, /SemioBrepMutation::SetSnapshot\(payload\)[^]*?ArtifactPreparedOperationSource::HexJson/u);
  assert.ok(existsSync(join(binary, "🫳️borrowed/🦀️.rs")), "original Brep snapshot must publish bounded ordinal JSON nodes");
  console.log("[DEBUG] installed original typed Brep SetSnapshot source callback exists");
}

if (import.meta.main) {
  assert.equal(process.argv[2] ?? "oracle", "oracle");
  oracle();
}
